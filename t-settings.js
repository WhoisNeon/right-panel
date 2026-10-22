/* Self-check for the Settings page: the key capture and the page sizing.
 *
 * It reads the real ui.html and runs the real binding block, so it cannot drift from the panel.
 * Run it with `node t-settings.js`; it exits non-zero on the first thing that is wrong.
 *
 * The key capture is the interesting one: a bare modifier is a whole binding (the tray used to
 * offer Ctrl / Shift / Space on their own), and it must not turn every Ctrl + X into "Ctrl".
 */
const fs = require('fs');
const vm = require('vm');
const path = require('path');

const html = fs.readFileSync(path.join(__dirname, 'src/ui.html'), 'utf8');
const main = fs.readFileSync(path.join(__dirname, 'src/main.rs'), 'utf8');
const win = fs.readFileSync(path.join(__dirname, 'src/sys/windows.rs'), 'utf8');

let failed = 0;
const eq = (what, got, want) => {
  const a = JSON.stringify(got), b = JSON.stringify(want);
  if (a !== b) { failed++; console.error(`FAIL ${what}\n  got  ${a}\n  want ${b}`); }
  else console.log(`ok   ${what}`);
};
const ok = (what, cond) => ok0(what, cond);
function ok0(what, cond) {
  if (cond) console.log(`ok   ${what}`);
  else { failed++; console.error(`FAIL ${what}`); }
}

/* ---- key capture: lift the real block out of the panel and run it ---- */
function slice(from, to) {
  const a = html.indexOf(from), b = html.indexOf(to, a);
  if (a < 0 || b < 0) throw new Error(`cannot find "${from}" … "${to}" in ui.html`);
  return html.slice(a, b);
}
const capture = slice('let listening = null;', "$('#bindOpen').onclick");
const KEY_MODS = slice('const KEY_MODS =', 'const KEY_PRETTY');

/* the block wires itself to the page's keydown; the test drives bindKey / keyup directly, so the
   listeners are collected and thrown away rather than connected to anything */
const env = {
  KEY_MODS: vm.runInNewContext(KEY_MODS + '; KEY_MODS'),
  S: {}, save() {}, renderBinds() {}, pushBinds() {}, send() {},
  addEventListener() {},
  $: () => ({ onclick: null }),
};
/* the panel's keyup listener is collected here so the test can fire it by hand */
env.__keyup = null;
env.addEventListener = (type, fn) => { if (type === 'keyup') env.__keyup = fn; };
vm.createContext(env);
vm.runInContext(capture, env, { filename: 'ui.html key capture' });
if (!env.__keyup) { console.error('FAIL the panel never wired its keyup listener'); process.exit(1); }

/* the event object is made inside the context: a plain object from out here carries this
   realm's prototypes, and the panel's code is none the wiser but the methods have to survive */
const key = (k, mods = {}) => `{ key: ${JSON.stringify(k)}, preventDefault() {}, stopPropagation() {}, getModifierState: m => ${JSON.stringify(Object.keys(mods))}.includes(m) }`;
/* listening / heldMod are `let` in the panel's script, so they live in the context's global
   lexical scope rather than on the sandbox object: setup and reads go through the same context. */
const run = (code) => vm.runInContext(code, env);
const start = (target = 'open') => run(`listening = ${JSON.stringify(target)}; heldMod = null; S.openKeys = []; S.binds = {};`);
const bind = (k, mods) => run(`bindKey(${key(k, mods)})`);
const release = (k) => run(`__keyup(${key(k)})`);
const read = (expr) => JSON.parse(run(`JSON.stringify(${expr})`));

start();
bind('q');
eq('a bare letter binds', read('S.openKeys'), ['Q']);
eq('the capture stops after one key', read('listening'), null);

start();
bind('Control', { Control: true });
eq('Ctrl alone does not commit on keydown', read('listening'), 'open');
release('Control');
eq('a bare modifier binds when released', read('S.openKeys'), ['Control']);

start();
bind('Control', { Control: true });
bind('q', { Control: true });
release('Control');
eq('Ctrl + Q stays a combination', read('S.openKeys'), ['Control', 'Q']);

start();
bind('Escape');
eq('Escape clears the binding', read('S.openKeys'), []);

start();
bind('F8');
eq('a function key binds on its own', read('S.openKeys'), ['F8']);

start('calc');
bind('Space');
eq('a tool key goes to its own binding', read('S.binds'), { calc: ['Space'] });

/* ---- settings pages are not pinned to each other ---- */
const equalise = slice('function equaliseSettings', 'function moveTpill');
ok('the pages are not pinned to a shared floor', !/minHeight\s*=\s*floor/.test(equalise));
ok('a page carries no min-height of its own', /\.style\.minHeight = ''/.test(equalise));
ok('a page is still capped to the room the panel has', /maxHeight = room/.test(equalise));
ok('the keybinds page has room for another row', /#bindTools \{ max-height: 224px; \}/.test(html));
ok('the keybinds page offers Reset all', /id="resetBinds"/.test(html) && /#resetBinds'\)\.onclick/.test(html));
ok('Reset all clears the hold key and the tool binds', /S\.binds = \{\}; S\.openKeys = \[\];/.test(html));
ok('the danger state uses the circle alert icon', /LUC\[armed \? 'circle-alert' : 'rotate-ccw'\]/.test(html));

/* ---- the frame loop keeps a stable top ---- */
const frame = slice("if (panel) flyUi.style.maxHeight = 'none';", 'const Y = fy.step(dt)');
ok('the panel is measured without last frame’s cap', /if \(panel\) flyUi\.style\.maxHeight = 'none';/.test(frame));
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

/* ---- the shield owns a magnifier, not the app-start hourglass ---- */
ok('the eyedropper draws its own magnifier cursor', /fn magnifier_cursor/.test(win) && /CreateIconIndirect/.test(win));
ok('the crosshair is only the fallback', /made\.is_null\(\) \{ LoadCursorW\(std::ptr::null_mut\(\), IDC_CROSS\)/.test(win));

/* ---- British spelling is gone from the shipped text ---- */
for (const f of ['README.md', '.github/RELEASE_NOTES.md', 'docs/CHANGELOG.md', 'src/ui.html', 'src/sys/windows.rs']) {
  const s = fs.readFileSync(path.join(__dirname, f), 'utf8');
  ok(`${f} says "color", not "colour"`, !/colour/i.test(s));
}

/* ---- the changelog is text again, not a tod of UTF-16 ---- */
const changelog = fs.readFileSync(path.join(__dirname, 'docs/CHANGELOG.md'));
ok('the changelog is UTF-8 with no NULs', changelog.indexOf(0) < 0);
ok('the changelog still has every release', /## v1\.4\.27/.test(changelog.toString('utf8')) && /## v1\.4\.30/.test(changelog.toString('utf8')));

process.exit(failed ? 1 : 0);
