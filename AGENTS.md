# AGENTS.md

Notes for AI coding agents working on Right Panel. Read this before changing anything.

## What this is

A desktop side panel that lives on the edge of the screen and pours out when you reach for it.
Rust + [tao](https://github.com/tauri-apps/tao) + [wry](https://github.com/tauri-apps/wry); the
entire user interface is one HTML file with no build step and no framework.

## Layout

```
src/ui.html        the whole UI: markup, CSS and JS in one file (~3,800 lines)
src/main.rs        window, event loop, IPC handlers, watcher threads, release plumbing
src/util.rs        platform-neutral helpers: PNG encoder, QR matrix + SVG, tray pixels, hashing
src/sys/mod.rs     picks the platform module
src/sys/windows.rs Win32: clipboard, keys, registry, system info, eyedropper, meters
src/sys/unix.rs    the same surface for macOS and Linux; anything Windows-only is a stub
assets/            two bundled fonts (Twemoji, Arad) and the icons — nothing is fetched at runtime
docs/              images for the README, and the changelog
installer/         Inno Setup script for the Windows installer
.github/           release.yml (builds every target on a v* tag) and RELEASE_NOTES.md
```

`src/sys/windows.rs` and `src/sys/unix.rs` must expose the **same function signatures**. Adding a
function to one means adding it to the other; CI builds all four targets and will catch a miss.

## Conventions

**The UI is one file on purpose.** Do not split it, and do not add a build step or a framework.
Sections are marked with banner comments (`/* ---- calculator ---- */`) and are read in order.

**No network at runtime, except where a tool is explicitly about the internet.** Fonts, icons and
emoji are all inlined or bundled. Icons are Lucide bodies in the `LUC` table in `ui.html`; never
add an icon that is not in that table, and never load an icon from the network.

**No native tooltips.** `title=` is never used. The panel draws its own with `data-tip` and the
`.tip` element. No native `alert` / `confirm` / `prompt` either — they open a second window on
Linux and steal focus.

**Persian and Arabic are first-class.** The calendar, the search box and every text field flip
their caret to match what you type. Persian text uses the bundled Arad face; Latin is left to the
system font.

**IPC is one shape.** The panel calls `send(type, payload)`; Rust matches on the type string and
answers with `webview.evaluate_script("app.something(...)")`. When you add one, add the reply on
the `app` object too, and keep the two in step.

**Comments say why, not what.** The codebase comments the decisions — which of two possible
formulas is right and what went wrong with the other — rather than narrating the code.

## Things that have bitten before

- **A click that lands somewhere else.** The dock springs into place; a press can start on a tool
  and end on the flyout behind it, and the browser then fires no click on the tool. The pressed
  tool is remembered *by name*, not by node, because the dock re-renders as it settles.
- **The window is bigger than the panel.** It is a fixed 480×680 strip, and the panel is drawn
  inside it. The panel reports the shape it can actually be clicked on (`send('hit', …)`) and the
  rest of the strip is made click-through — if that ever gets stuck on, the panel is visible and
  completely dead, and the watcher thread gives the input back on its own.
- **Windows `GetPixel` returns `0x00BBGGRR`**, blue first. Reading it left to right swaps red and
  blue.
- **`key_code` must not trim its input.** A keybind on Space is written as a bare `" "`, and
  trimming it away left the panel reading the combination as "no key needed".
- **A Jalali year is ~1400 and a Gregorian one is ~2026.** Anything that tells the two apart has
  the inequality the easy way round and rejects every real date.
- **The system information is handed to the page once**, when the window is built. Reloading the
  page brings the *old* settings back, so never `location.reload()` to apply a change.

## Verifying a change

There is no Rust toolchain assumed locally and CI is the compiler, so:

1. `node check.js` — parses the UI and reports stray debug output.
2. `node check-luc.js` — every icon referenced is in the `LUC` table.
3. The test suites, one per area (`t-calc`, `t-jalali`, `t-new`, `t-num`, `t-pw`, `t-search`,
   `t-settings`, `t-text`, `t-units`, `t-zones`). They read the real source and drive the real
   handlers, so they cannot drift from it. **Add an assertion when you change behaviour** — a
   change with no test is not finished.
4. `node build-harness.js` then open `http://127.0.0.1:8731/` to drive the panel in a browser.
   Note that `requestAnimationFrame` does not run in a headless tab, so anything that animates
   cannot be checked there.
5. Push, then `gh workflow run release.yml --ref main --repo=WhoisNeon/right-panel` and watch the
   four build jobs. Rust changes are not verified until this is green.

## Cutting a release

1. Bump `version` in `Cargo.toml`.
2. Replace `.github/RELEASE_NOTES.md` with **only this release** — its download links must point
   at `…/releases/download/v<version>/<asset>` for the tag being cut, never at
   `/releases/latest/download/`, and it links to `docs/CHANGELOG.md` for everything before it.
3. Move the previous release's notes onto the top of `docs/CHANGELOG.md`.
4. Update the version badges in `README.md` (and the size badge if the artifact size moved).
5. Commit, push, tag `v<version>` and push the tag. The workflow builds all four targets and
   publishes the release from `RELEASE_NOTES.md`.