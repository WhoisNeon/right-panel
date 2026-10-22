/* Self-check for the Settings page: the key capture, the tab sizing and the eyedropper.
 *
 * It reads the real ui.html and runs the real binding block, so it cannot drift from the panel.
 * Run it with `node t-settings.js`; it exits non-zero on the first thing that is wrong.
 *
 * The key capture is the interesting one: a bare modifier is a whole binding (the tray used to
 * offer Ctrl / Shift / Space on their own), it must not turn every Ctrl + X into "Ctrl", and a
 * combination may only ever belong to one row.
 */
const fs = require('fs');
const vm = require('vm');
const path = require('path');

const root = __dirname;
const html = fs.readFileSync(path.join(root, 'src/ui.html'), 'utf8');
const main = fs.readFileSync(path.join(root, 'src/main.rs'), 'utf8');
const win = fs.readFileSync(path.join(root, 'src/sys/windows.rs'), 'utf8');
const cargo = fs.readFileSync(path.join(root, 'Cargo.toml'), 'utf8');
const lock = fs.readFileSync(path.join(root, 'Cargo.lock'), 'utf8');
const readme = fs.readFileSync(path.join(root, 'README.md'), 'utf8');
const notes = fs.readFileSync(path.join(root, '.github/RELEASE_NOTES.md'), 'utf8');
const changelog = fs.readFileSync(path.join(root, 'docs/CHANGELOG.md'), 'utf8');
const version = (cargo.match(/^version = "([^"]+)"/m) || [])[1];

let failed = 0;
const eq = (what, got, want) => {
  const a = JSON.stringify(got), b = JSON.stringify(want);
  if (a !== b) { failed++; console.error(`FAIL ${what}\n  got  ${a}\n  want ${b}`); }
  else console.log(`ok   ${what}`);
};
function ok(what, cond) {
  if (cond) console.log(`ok   ${what}`);
  else { failed++; console.error(`FAIL ${what}`); }
}

function slice(from, to) {
  const a = html.indexOf(from), b = html.indexOf(to, a);
  if (a < 0 || b < 0) throw new Error(`cannot find "${from}" ... "${to}" in ui.html`);
  return html.slice(a, b);
}

/* ---- key capture: lift the real block out of the panel and run it ---- */
/* The block runs from where the capture state starts to where the buttons that start one take
   over; the vocabulary it reads (KEY_MODS, bindLabel, bindKeyOf) sits just above it. Everything
   the block reaches for is declared *inside* the context: a vm context keeps its own copy of any
   object or array handed to it from here, so state written into that copy would never come back
   to the assertions. `notices` is made in there for the same reason. */
const capture = slice('let listening = null;', "$('#bindOpen').onclick");
const vocab = slice('const KEY_MODS =', 'const bindShort =');
const prelude = `
  var notices = [];
  var elements = {};
  function element(sel) {
    if (!elements[sel]) elements[sel] = { onclick: null, textContent: '', dataset: {}, innerHTML: '', classList: { toggle() {} } };
    return elements[sel];
  }
  var $ = element;
  var I = () => '';
  var esc = s => String(s);
  var LUC = {};
  var W = { calc: { name: 'Calculator' }, timer: { name: 'Timer' } };
  var EYE = { id: 'eyedrop', name: 'Eyedropper' };
  var S = { binds: {}, openKeys: [] };
  var save = () => {};
  var send = () => {};
  var renderBinds = () => {};
  var toast = m => notices.push(m);
  var app = { close() {} };
  var keyupFn = null;
  function addEventListener(type, fn) { if (type === 'keyup') keyupFn = fn; }
  ${vocab}
`;
const ctx = vm.createContext({});
vm.runInContext(prelude, ctx);
vm.runInContext(capture, ctx, { filename: 'ui.html key capture' });

const run = code => vm.runInContext(code, ctx);
const read = expr => JSON.parse(run(`JSON.stringify(${expr})`));
ok('the capture block carries the duplicate check', /function takeBind/.test(capture) && /const bindOwner/.test(capture));
ok('the panel wired its keyup listener', run('typeof keyupFn') === 'function');

/* one combination, from a clean shelf */
const reset = (openKeys = []) =>
  run(`listening = null; heldMod = null; S.openKeys = ${JSON.stringify(openKeys)}; S.binds = {}; notices.length = 0;`);
/* the row a capture is aimed at; what is already bound to the other rows is left alone */
const aim = target => run(`listening = ${JSON.stringify(target)}; heldMod = null;`);
const event = (k, mods = {}) =>
  `{ key: ${JSON.stringify(k)}, preventDefault() {}, stopPropagation() {}, getModifierState: m => ${JSON.stringify(Object.keys(mods))}.includes(m) }`;
const bind = (k, mods) => run(`bindKey(${event(k, mods)})`);
const release = k => run(`keyupFn(${event(k)})`);

reset();
aim('open');
bind('q');
eq('a bare letter binds', read('S.openKeys'), ['Q']);
eq('the capture stops after one key', read('listening'), null);

reset();
aim('open');
bind('Control', { Control: true });
eq('Ctrl alone does not commit on keydown', read('listening'), 'open');
release('Control');
eq('a bare modifier binds when released', read('S.openKeys'), ['Control']);

reset();
aim('open');
bind('Control', { Control: true });
bind('q', { Control: true });
release('Control');
eq('Ctrl + Q stays a combination', read('S.openKeys'), ['Control', 'Q']);

reset();
aim('open');
bind('Escape');
eq('Escape clears the binding', read('S.openKeys'), []);

reset();
aim('open');
bind('F8');
eq('a function key binds on its own', read('S.openKeys'), ['F8']);

reset();
aim('calc');
bind('Space');
eq('a tool key goes to its own binding', read('S.binds'), { calc: ['Space'] });

/* ---- one combination, one owner ---- */
reset();
aim('calc');
bind('a');
aim('timer');
bind('A');
eq('the same key is not bound to two tools', read('S.binds'), { timer: ['A'] });
eq('the row that had it is told it let go', read('notices'), ['Calculator unbound from A']);

reset();
aim('calc');
bind('k', { Control: true, Shift: true });
aim('timer');
bind('K', { Shift: true, Control: true });
eq('the order the keys went down in does not make a second binding', read('Object.values(S.binds)'), [['Control', 'Shift', 'K']]);

reset(['Control']);
aim('timer');
bind('Control', { Control: true });
release('Control');
eq('a tool can take the hold key, one owner at a time', read('S.binds'), { timer: ['Control'] });
eq('the hold key lets go', read('S.openKeys'), []);
eq('and says so', read('notices'), ['The hold key unbound from Ctrl']);

reset();
run('S.binds = { timer: ["Control"] }');
aim('open');
bind('Control', { Control: true });
release('Control');
eq('the hold key does not take a tool key', read('S.openKeys'), ['Control']);
eq('the tool that had it lets go', read('S.binds'), {});

reset();
aim('calc');
bind('a');
aim('calc');
bind('A');
eq('a row may rebind its own key', read('S.binds'), { calc: ['A'] });

/* ---- settings pages are not pinned to each other ---- */
const equalise = slice('function equaliseSettings', 'function moveTpill');
const floor = slice('function keyPageFloor', 'function equaliseSettings');
ok('the pages are not pinned to a shared floor', !/minHeight\s*=\s*floor/.test(equalise));
ok('a page starts with no min-height of its own', /p\.style\.minHeight = ''/.test(equalise));
ok('a page is still capped to the room the panel has', /maxHeight = room/.test(equalise));
ok('only Keybinds gets a floor', /keyPageFloor\(\)/.test(equalise));
ok('the Keybinds page takes the Widgets page height', /widgets\.offsetHeight/.test(floor) && /keys\.style\.minHeight = h/.test(floor));
ok('the Widgets page is measured on its own', /widgets\.classList\.add\('on'\)/.test(floor) && /keys\.classList\.remove\('on'\)/.test(floor));
ok('the keybinds list still scrolls at six rows', /#bindTools \{ max-height: 224px; \}/.test(html));
ok('the keybinds list is filled before the page is measured', /addEventListener\('load', \(\) => \{ renderBinds\(\); equaliseSettings\(\); \}/.test(html));
ok('the keybinds page offers Reset all', /id="resetBinds"/.test(html) && /#resetBinds'\)\.onclick/.test(html));
ok('Reset all clears the hold key and the tool binds', /S\.binds = \{\}; S\.openKeys = \[\];/.test(html));
ok('the danger state uses the circle alert icon', /LUC\[armed \? 'circle-alert' : 'rotate-ccw'\]/.test(html));

/* ---- the frame loop keeps a stable top ---- */
const frame = slice("if (panel) flyUi.style.maxHeight = 'none';", 'const Y = fy.step(dt)');
ok("the panel is measured without last frame's cap", /if \(panel\) flyUi\.style\.maxHeight = 'none';/.test(frame));
ok('a wanted top is not ratcheted by a tall page', !/flyTop = Math\.min\(/.test(frame));
ok('an overflowing page is pulled up by its full overflow', /flyTop \+ flyH \+ 10 - innerHeight/.test(frame) && !/over\) \/ 2/.test(frame));

/* ---- the tray no longer mirrors the hold key ---- */
for (const gone of ['Hold key to open', 'set_open_mod', 'OPEN_MOD', 'openmod-']) {
  ok(`the tray dropped "${gone}"`, !main.includes(gone));
}
ok('the tray offers Reset to default', /"reset", "Reset to default"/.test(main));
ok('the tray reset goes through the panel, so it takes two taps', main.includes('app.armReset()'));
ok('the panel exposes armReset', /app\.armReset\s*=/.test(html));

/* ---- the network tool has no open-domain button ---- */
ok('the network tool has no open-domain button', !/netOpen|NET_OPEN_ROW|rowopen/.test(html));
ok('the Domain row is still read out', /connection\?\.domain/.test(html));

/* ---- the eyedropper magnifies what is under the pointer ---- */
ok('the native side copies a magnified screen block', /fn screen_region/.test(win) && /fn pick_magnifier/.test(win));
ok('the loupe travels to the page as a picture', /app\.picking\(\.\.\./.test(main));
ok('the panel corner is read from the crate root', /crate::PANEL_X/.test(win) && !/PANEL_AT/.test(win));
ok('the loupe is a fixed 150px box, not the whole window', /#pickMag \{ position: fixed; width: 150px; height: 150px;/.test(html));
ok('the loupe is placed inside the window', /innerWidth - LOUPE/.test(html) && /innerHeight - LOUPE/.test(html));
ok('the corner chip steps aside once the loupe is up', /pickHint\.hidden = true;/.test(html));
ok('Escape gives the overlay back', /app\.pickCancel/.test(main) && /app\.pickCancel = /.test(html));
ok('the capture window shows the plain arrow, not the pixel-covering crosshair',
  /cursor: LoadCursorW\(std::ptr::null_mut\(\), IDC_ARROW\)/.test(win) && !/IDC_CROSS/.test(win));

/* ---- the release being cut ---- */
ok('Cargo.toml and Cargo.lock agree on the version',
  new RegExp(`name = "right-panel"\\r?\\nversion = "${version.replace(/\./g, '\\.')}"`).test(lock));
ok('the README badges say the version being cut', new RegExp(`v${version.replace(/\./g, '\\.')}`).test(readme));
ok('the notes link this tag, never /latest/', notes.includes(`releases/download/v${version}/`) && !notes.includes('/releases/latest/download/'));
ok('the previous release is what the changelog still leads with', changelog.includes('## v1.4.32') && !changelog.includes(`## v${version}`));
ok('the changelog is UTF-8 text with no NULs', !changelog.includes('\u0000'));

/* ---- British spelling is gone from the shipped text ---- */
for (const f of ['README.md', '.github/RELEASE_NOTES.md', 'docs/CHANGELOG.md', 'src/ui.html', 'src/sys/windows.rs']) {
  const s = fs.readFileSync(path.join(root, f), 'utf8');
  ok(`${f} says "color", not "colour"`, !/colour/i.test(s));
}

process.exit(failed ? 1 : 0);