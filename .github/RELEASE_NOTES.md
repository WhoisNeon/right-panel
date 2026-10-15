![Right Panel](https://raw.githubusercontent.com/WhoisNeon/right-panel/main/docs/preview-laptop.png)

### Downloads
| Platform | Get it |
|---|---|
| **Windows** (recommended) | **RightPanel-Setup.exe** installer · or `RightPanel-portable.exe` / `RightPanel-windows-x64.zip` |
| **macOS** *(beta)* | `RightPanel-macos-arm64.dmg` (Apple Silicon) · `RightPanel-macos-x64.dmg` (Intel). Right-click → Open the first time |
| **Linux** *(beta)* | `RightPanel-linux-amd64.deb` · `RightPanel-linux-x64.tar.gz` (X11 + WebKitGTK 4.1) |

### New in 1.4.6
- **Units panel rebuilt** — a much larger set: length (mm · cm · m · km · in · ft · yd · mi), mass (mg · g · kg · oz · lb), volume (ml · l · fl oz · gal), **data sizes (b · KB · MB · GB · TB · PB)**, speed (km/h · mph · m/s · knot), temperature (C · F · K) and frequency (BPM · Hz). Accepts `1,024 KB`, `1.5mb`, `°C` and `km/h`
- **Text tools accept typed text** — no longer clipboard-only; an empty box still falls back to the clipboard
- **Clipboard images show their file size** next to the resolution
- **Note titles are editable** — double-click a tab to name it; unnamed notes keep showing their first line
- **Settings rows: proper icons** — the Dock/More move buttons and the drag handle are drawn as vectors and sit where they belong
- **Back button** centres its arrow and has more breathing room from the panel title
- **Timer presets wrap** instead of the 45-minute button escaping the panel
- **Panels no longer close the instant they open** — the focus change caused by opening one no longer counts as clicking away
- **Correct version number** — the About panel and installer now report the real release instead of being stuck at 1.4.0

### New in 1.4.4
- **Choose how the panel opens** (*Settings → General*): any edge, or only while holding **Ctrl**, **Shift** or **Space**
- **No more hover tooltips** — labels moved off `title` so nothing pops up over a button, while screen readers still get the names
- **Generate panel stays inside its box** — long values like lorem ipsum scroll instead of spilling out
- **Panel icons are properly centred** — the back arrow, the ★ pin and the × remove are now real icons, drawn on the same footing as every other icon
- **Persian and Arabic text renders in Arad**, with Latin left to your system font
- The binary grew to ~2.3 MB: the emoji and Persian fonts now ship inside it

### New in 1.4.3
- **Persian and Arabic text now actually renders in Arad** — it was being drawn by the Windows Arabic fallback instead
- **Panel icons line up again** — the back arrow, ★ pin and × close keep the text font's metrics instead of borrowing an emoji font's
- **Clipboard rows stay put on hover** — they no longer slide sideways or disappear under their neighbours
- **Generate panel fits** — the long lorem-ipsum values truncate instead of pushing past the panel edge

### New in 1.4.2
- **Emoji:** the panel no longer closes when you pick an emoji — browse and pick several in a row
- **Emoji:** country flags fit the grid properly, and the category row no longer overflows the picker
- **Media:** the play button flips to a pause icon when you press it
- **Generate:** UUIDs work again — the browser API they used is unavailable inside the panel, so they are built from the random-number source instead
- Fixed settings silently resetting on every launch

### New in 1.4.0
- **Country flags** in the emoji picker — all 259 of them, searchable by name in English and Persian
- **Persian search** in the emoji panel — `خنده` finds 😂, `آتش` finds 🔥, `ایران` finds 🇮🇷, `آمریکا` finds 🇺🇸
- **Persian & Arabic text** now renders in the Arad typeface everywhere in the panel, and any text field flips its caret to match what you're typing
- **New emoji look** — [Twemoji](https://github.com/jdecked/twemoji) graphics in the picker, falling back to your system emoji font for anything it doesn't cover
- **Updates now come from this fork** rather than the upstream repository

### New in 1.3.1
- **Linux:** the drop-down settings (panel edge, monitor, animation speed, world-clock city) no longer close the panel when you open them — they now use in-panel menus

### New in 1.3.0
- **Timer: set any time** — type minutes and seconds, plus more presets (15m, 45m, 60m) (#5)
- **Linux: text fields work again** — the panel now takes keyboard focus for search, notes, calculator… (#4)
- **Multi-monitor fix** — when another screen sits past the panel's edge, the pointer crossing between screens no longer pops the panel open in the middle of your desktop; rest on the edge for a moment to open it. Panel size now follows each monitor's own scaling
- **Animation speed** setting (Settings → General): Slow · Relaxed · Normal · Fast · Very fast

### New in 1.2.0
- **Drag files and folders onto the panel** to pin them
- **Clipboard history now keeps images**, with ★ pins that survive a restart
- **Pick the edge** (left or right) and the **monitor** the panel lives on
- **Several notes** in tabs instead of one
- Settings can add a **folder** as well as an app

### What's inside
Emoji picker with country flags · clipboard history with pins · color picker & eyedropper · screenshot · notes · pinned apps · web search · snippets · calculator · units · password & generators · timer · stopwatch · world clock · media keys · text tools · keep awake · screen off · lock · pin window on top.
Persian and Arabic throughout the interface and emoji search. Drag tools between the dock and the More grid, 15 themes, liquid animations, tray icon, start at login.
