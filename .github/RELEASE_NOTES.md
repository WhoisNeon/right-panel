![Right Panel](https://raw.githubusercontent.com/WhoisNeon/right-panel/main/docs/preview-laptop.png)

### Downloads
| Platform | Get it |
|---|---|
| **Windows** (recommended) | **[RightPanel-Setup.exe](https://github.com/WhoisNeon/right-panel/releases/latest/download/RightPanel-Setup.exe)** installer · or [RightPanel-portable.exe](https://github.com/WhoisNeon/right-panel/releases/latest/download/RightPanel-portable.exe) / [RightPanel-windows-x64.zip](https://github.com/WhoisNeon/right-panel/releases/latest/download/RightPanel-windows-x64.zip) |
| **macOS** *(beta)* | [RightPanel-macos-arm64.dmg](https://github.com/WhoisNeon/right-panel/releases/latest/download/RightPanel-macos-arm64.dmg) (Apple Silicon) · [RightPanel-macos-x64.dmg](https://github.com/WhoisNeon/right-panel/releases/latest/download/RightPanel-macos-x64.dmg) (Intel). Right-click → Open the first time |
| **Linux** *(beta)* | [RightPanel-linux-amd64.deb](https://github.com/WhoisNeon/right-panel/releases/latest/download/RightPanel-linux-amd64.deb) · [RightPanel-linux-x64.tar.gz](https://github.com/WhoisNeon/right-panel/releases/latest/download/RightPanel-linux-x64.tar.gz) (X11 + WebKitGTK 4.1) |

*Every release keeps its own files too — see [all releases](https://github.com/WhoisNeon/right-panel/releases).*

### New in 1.4.23
- **Reset to default no longer corrupts your settings** — 1.4.22 reloaded the panel the instant it asked to save, which could beat the write and leave a half-written `settings.json` behind. Deleting `%APPDATA%\RightPanel` fixed it; it should never have needed that. The panel now waits for the native side to confirm the file is on disk, then reloads
- **The panel opens on the edge again after a reset** — the reloaded page starts closed, but the native side still thought it was open and so ignored the hover. The page now says hello on every load and the two agree
- **Only the Jalali numbers are Persian in the calendar** — in the Gregorian month the big day numbers are English and the Jalali day underneath is Farsi; in the Jalali month it is the other way round
- **Dropdown lists measure the box that clips them** — on a short panel a list could be cut off part-way instead of simply opening downward

### New in 1.4.22
- **The dropdowns stay inside the panel** — a Settings list near the foot of the panel used to open straight out through the bottom edge, because it measured the room against the screen rather than the panel. Each list now flips up or caps its height to the room it actually has
- **The time zone picker got shorter** — the zones you already added are just the city name and a ✕, so a list of ten no longer fills the Add page. The clocks themselves are still on the World clock page, one click back
- **Reset to default** — the last button in Settings → General puts everything back: your dock and More layout, custom themes, snippets, time zones and preferences. It takes two taps, because it throws all of that away, and it is not a native dialog (those are a second window on Linux)

### New in 1.4.21
- **One "Settings" again** — the tool that hands over to Windows is back to being called **Win settings**, so the list no longer has two entries with the same name and only one of them opens this panel
- **The snippet placeholders are in reading order** — `date`, `jalali_date`, `time`, `time_12h`, `weekday`, `weekday_fa`, `month`, `month_fa`, `jalali_month`, `jalali_month_fa`, `year`, `jalali_year`, and `clipboard` last. Each English one is followed by its Persian twin

### New in 1.4.20
- **The settings folds actually close** — 1.4.19 could look shut but wasn't: closing **Panel** or **System** turned the rows invisible and left the space they took up behind, so the page kept a gap where the settings used to be. Both folds now close the way the search engine groups do
- **The General page is a page again** — the same change had wrapped the System section, the version line and the Quit button *inside* the Panel section's box, which is why the rows sat oddly and the spacing came out wrong. Everything is back at page level

### New in 1.4.19
- **Sections fold shut** — the snippet placeholders and the two Settings groups (**Panel**, **System**) now close like the search engine groups do, and each one remembers whether you left it open or shut
- **The world clock got a proper Add page** — *Add* opens the picker as its own full panel with the search box in it, and the clock list behind it is just the clock: no search, no ✕. The **✕ Close** button works here now, so the picker is where your cities are added *and* removed
- **The QR preview is the same picture you copy** — it used to be redrawn in the panel from the module matrix, and that matrix never arrived intact, so the preview was blank while the clipboard was fine. The preview is now the exact bitmap that lands on your clipboard, drawn as an image, and the two can no longer disagree
- **The calendar boxes hold what a date can hold** — the year takes four digits, month and day two, `13` becomes `12`, and the day stops at 29, 30 or 31 depending on the month you typed next to it (February included)
- **The calendar is written in Persian digits** — the day numbers in the grid are now ۰۱۲۳; the two cards at the bottom keep the digits you chose in Settings
- **Performance shows more** — RAM in GB under its gauge, plus how long the machine has been up. The hint reads *sampled once a second* now that the label above it says CPU and RAM
- **Six date formats, in one order** — YYYY/MM/DD, YYYY-MM-DD, MM/DD/YYYY, MM-DD-YYYY, DD/MM/YYYY, DD-MM-YYYY
- **Five more snippet placeholders** — `{{weekday_fa}}`, `{{month_fa}}`, `{{jalali_month_fa}}`, `{{year}}` and `{{jalali_year}}` (you wrote `jalili__month_fa`; it is one l and no gap)
- Smaller things — the weekday row over the calendar is set in Arad, **UUID** moved above the Lorem lines in Generate, the icons on the header links are a size smaller, and today in the calendar only takes the accent colour when it is not the selected day

### New in 1.4.18
- **The world clock reads in UTC offsets** — each city now shows `UTC+03:30` or `UTC-04:00` instead of "Today, +3h", read per moment so daylight saving is already right
- **Ten time zones without scrolling** (was six), each row now **city · clock**, and the add picker has room to breathe
- **The add picker gets out of the way** — while it is open the clock hides, the button turns into **✕ Close**, and the zones you already have sit at the top in grey so you can see what is there
- **The calendar opens in Gregorian now** — click the month name to flip between the two, and it remembers which one you left it on. The date at the bottom is three boxes (year, month, day) read in whichever calendar you are looking at, and both cards copy on click
- **Two date formats, one place** — Settings → General has *Gregorian date* and *Jalali date*, each with DD/MM/YYYY, MM/DD/YYYY, YYYY-MM-DD, DD.MM.YYYY or YYYY/MM/DD. They apply to Generate, the calendar and the snippet placeholders at once, and the two calendars keep their own order
- **Snippets are editable** — a pencil before the bin opens the row for editing, Enter saves and Escape backs out
- **Snippets show what they will copy** — a snippet saved as `hi {{date}}` reads `hi 17/10/2026` in the list, with four more placeholders on the box: `{{time_12h}}`, `{{weekday}}`, `{{month}}`, `{{jalali_month}}`
- **The QR preview draws again** — the code matrix now crosses the bridge as one string instead of a bool array, which was arriving empty and leaving a white square
- **More opens when you click it right after the panel appears** — the pointer opening a tool and the click that followed were reading as open-then-close
- **System monitor is now Performance**, with a gauge of its own rather than Task Manager's icon, and *Win settings* is just *Settings*

### New in 1.4.17
- **A calendar, in both calendars at once** — a month grid where every day carries its Jalali number big and its Gregorian number small, so you never have to convert anything in your head. It opens on the Iranian week (Saturday first), has Today and month arrows, and marks today. Below it a box takes either calendar's date — `2026-10-16` or `1405-07-24`, with `/` or `-` — and shows you the other one, in both directions
- **A QR code generator** — type text, a link, a Wi-Fi password, and it is drawn as a sharp inline SVG with the quiet zone already in. *Copy PNG* puts it on the clipboard as an image you can paste straight into a document or a message
- **A system monitor** — CPU and RAM as live sparklines, sampled once a second and only while the panel is open, so it costs nothing when you are not looking

### New in 1.4.16
- **The calendar picker waits to be asked for** — the world clock used to show its whole search box and zone list the moment it opened, which was most of the panel. There is now an **+ Add** button in the header and the picker only appears when you press it. The box says what it wants: "City or Area/City, e.g. Asia/Tehran"
- **The world clock has seconds, and a 12/24h switch** — every zone shows `HH:MM:SS`, and the new **24h** button in the header flips the lot to 12-hour with AM/PM. Your choice is remembered
- **Snippets understand placeholders** — write `{{date}}`, `{{jalali_date}}`, `{{time}}` or `{{clipboard}}` in a snippet and it is filled in when you copy it, so the saved snippet stays the template you typed. The four names sit under the box as buttons — press one to drop it in
- **Generate grew a Jalali date** — the Iranian date (`1405-07-24`) sits right under the Gregorian one, and UUID moved down to the bottom of the list where the other random things live
- **Tighter rows** — Settings rows have an 8px gap between their label and their control, and the header buttons are down to a 4px gap
- **The Jalali date is right to the day** — it is the published 33-year-cycle arithmetic rather than an approximation, and it holds up over an 80-year, day-by-day round trip (29,220 dates, no repeats and no skips)

### New in 1.4.15
- **The panel slides and squeezes open again** — this is the 1.4.13 animation, back on purpose: the panel comes out of the screen edge and stretches open about its inner edge. (1.4.14 changed it to "only the height pours")
- **A new light theme, Linen** — the last of the presets, a warm off-white with a brown accent. Every other colour in it is derived, the same as the rest
- **Every time zone is in the clock now** — the picker was a fixed list of 25. It is now all 418 zones the engine knows, as `Asia/Tehran`, with a search box: type `tehran`, `asia/teh` or just `teh`. The zones you already added stay out of the list
- **The password stops moving on its own** — opening the panel used to roll a brand new one, so the one you had copied was quietly gone. It now changes only when you press Regenerate, flip a character set, or move the length slider
- **The timer picks the unit you point at** — hover the hours, minutes or seconds and it is the one the wheel and the arrow keys will change, no click needed. The numbers roll over too: scrolling down from `00` lands on `99` / `59`, and scrolling up from the top wraps back to `00`
- **The emoji grid stops showing a scrollbar** — hovering the last column armed a horizontal scrollbar, and the first column was sliced against the panel edge. The grid now pads itself so the hover scale has room, and six rows fit with no scrollbar at all
- **Pinning a window no longer hides the panel** — pinning raised the window above the panel and buried it. The panel raises itself again right after, and it remembers which window you pinned so unpinning hits the same one
- **Pin window is a toggle with a light** — it behaves like Keep awake now: the dot on the button tells you whether a window is pinned, instead of you having to guess
- **Smaller fixes** — the world-clock add button reads "Add", and an empty dropdown no longer throws when there is nothing to show

### New in 1.4.14
- **The timer is its own input now** — the hours / minutes / seconds boxes are gone. Click a unit on the display and scroll the wheel, press ↑ / ↓, or just type digits; it always reads `00:05:00`, no matter how short the timer is
- **Nothing moves sideways when the panel opens** — it used to slide in from the screen edge while it stretched wider, which dragged every row about 160px to the left and sprang it back. The panel is at full width from the first frame now, and only pours open downwards
- **Settings keeps its top** — switching Widgets / Theme / General used to jump the whole panel up and down the screen. The top edge now stays where it was and only the bottom edge moves; if a page is taller than the room under it, the page scrolls inside instead of shoving the panel up
- **The Units dropdowns take the click** — the list looked right, but clicks passed straight through it to the row underneath, so picking a unit did nothing. Both pickers now catch it and close
- **The zone ✕ works every time** — clicking it usually hit the empty space around the glyph and did nothing
- **The add buttons match** — "+ App", "+ Add" (Folders, Theme) and the Clear buttons drew their icons at 12px with a wide gap; they are 16px now, the same size as the "Add a city…" plus, and sit centred on their labels. "Add a city…" is centred too, with its plus level with the text instead of riding above it
- **Two more text tools** — *Snake case* (Hello World-Test → hello_world_test) and *Sort lines* (case-insensitive, one line each)
- **Snippets can be cleared** — a Clear (bin) button in the Snippets header, like the clipboard's. It clears straight away with no confirmation, the same as the clipboard does
- **The keepers hold more** — the recent-emoji row keeps 42 (was 28), pinned clips keep 50 (was 30), and the clipboard no longer throws anything away after 50 clips
- **Translate picks the direction for you** — English text targets Farsi, anything else targets English
- **Smaller fixes** — content keeps 2px from the scrollbar instead of 4px, and the timer's screen-reader label reads out the current time

### New in 1.4.13
- **Settings → General explains itself** — every option now has a one-line hint under its name ("Which screen edge the tab sits on", "How fast the panel and its menus move"), the options are grouped under *Panel* and *System*, and "Open panel" is now "Hold to open"
- **The hold key is in the tray menu too** — right-click the tray icon → *Hold key to open* offers the same No key / Ctrl / Shift / Space, and it stays in step with the one in Settings whichever you use
- **Settings slides between tabs** — the Widgets / Theme / General background is a single pill that glides across instead of snapping
- **Nothing jumps when the panel opens** — the Settings rows used to shove about 30px left and spring back as the panel grew; the open animation no longer overshoots sideways
- **The Units dropdowns are readable** — the list was filled with exactly the same colour as the rows underneath, so it looked see-through and its extent was invisible; it now uses the panel's floating-menu surface
- **Translate knows which way you're going** — searching with Translate targets Farsi when you type English, and English when you type Farsi
- **Password and Token generators** — Generate has a 16-character password (all four character sets) and a 128-bit hex token
- **The clipboard tool pastes** — clicking a clip pastes it into the app you came from and closes the panel, instead of only copying it
- **Smaller fixes** — the "+ App" button's icon and label sit on the same line, the colour picker closes with the panel, the emoji tool no longer flashes a "Typed" toast, "Add a city…" carries a proper plus icon, the timer keeps just 5m / 10m / 15m / 30m / 1hr centred on one row, and the important controls carry aria-labels for screen readers

### New in 1.4.12
- **Right-click a tool** — in the dock or in More, right-click any tool for *Move to dock / Move into More* and *Disable tool*; Settings → Widgets switches it back on. Dragging a tool onto the More icon also moves it to More
- **Copy, never paste** — everything you click lands on the clipboard: snippets, Generate, the text tools, the notes. The "Paste on click" setting is gone. Clicking an emoji types it straight into the app you came from, with no clipboard round-trip
- **Menus behave** — the WebView's own right-click menu is gone, the dropdowns and the colour picker fade and scale open and shut, the dropdown list no longer looks see-through, and the caret next to a dropdown label is gone
- **Icons everywhere** — New, Copy, Delete, Add, Clear, Save, Cancel, Regenerate and Quit carry icons, and the calculator's ± key is a proper glyph instead of text
- **Smaller fixes** — the emoji category tabs show their icons again, the selected theme swatch is no longer sliced at the top, the toggled Keep Awake tile stays readable on hover, and Settings slides between Widgets, Theme and General

### New in 1.4.11
- **Build your own theme** — Settings → Look has a theme builder: name it, pick a background and an accent, and every other colour (raised surfaces, separators, muted labels, hover and selected states) is derived from those two. The text colour on the accent is chosen for you — black or white, whichever reads better on the colour you picked
- **A real colour picker** — choose a colour by eye from the square or type a hex code; it is used wherever a colour is chosen, including the Colour tool, and it shows the same value as hex, rgb and hsl
- **Every icon is a Lucide icon now** — the emoji-category tabs, the toasts, the calculator keypad, the media controls and the widget tiles moved off emoji and one-off drawings onto one consistent icon set, all drawn inline with nothing fetched at runtime
- **Drag a tool into the More panel** — dragging a tile out of the dock and over the More button now opens its panel so you can drop the tool straight in, and the stale "drag a tile on to the dock" hint is gone from the More header
- **The dock and More lists line up** — Settings → Widgets showed five rows in the dock and four and a half in More; both show exactly five now, and the page scrolls if your window is short
- **Keep Awake is readable in More again** — its text used to turn black while the tile background stayed dark, so it disappeared; the tile now uses the theme's inverted surface, exactly as the dock does
- **Search groups glide open and shut** — collapsing a category animates instead of snapping, and Neshan's icon is filled like the other engine marks
- **Smaller, tidier controls** — the volume buttons are a little smaller than mute, the move and show/hide buttons in the Widgets rows sit closer together, and content keeps a small gap from the scrollbar
- **Only `=` wears the accent in the calculator** — the operators (`÷ × − +`) now sit on a raised surface that sets them apart from the digits
- **Custom dropdowns everywhere** — the engine, zone, unit and monitor pickers all use the panel's own dropdown instead of the operating system's
- **Settings and Emoji have titles**, like every other panel, and the toggles follow the theme instead of always being the same green

### New in 1.4.10
- **The calculator is a real Windows-Standard calculator now** — it keeps a running value instead of building a formula, so the display can only ever hold one number and no key sequence can produce something unparseable. `9-++6---` gives `15`, `2+3*4=` gives `20` (left to right, no precedence, exactly like Windows), `5..3` gives `5.3`, and `2+3=` then `=` again gives `8`
- **The keypad matches Windows Standard** — `CE` `C` `⌫` `÷` across the top, then `7 8 9 ×`, `4 5 6 −`, `1 2 3 +`, and `+/− 0 . =` underneath, all drawn as icons. Brackets and `^` are gone: they cannot work in a pad that evaluates as you type
- **You can see what you're working on** — a small line above the digits shows the pending sum (`9 −`), and the finished one after `=` (`2 + 3 = 4`)
- **Divide by zero says so** — `Cannot divide by zero`, or `Result is undefined` for `0 ÷ 0`, and the pad locks itself until you press `C` or `CE`
- **Typing still works** — `Enter` or `=` evaluates, `Backspace` deletes, `Delete` is `CE`, `c` is `C`, `F9` flips the sign, and `Escape` still closes the panel
- **The search tool was rebuilt** — engines sit in collapsible groups you can fold away, and every button carries its real brand mark (Simple Icons), drawn inline with nothing fetched at runtime
- **Three more engines, and Bing is gone** — Startpage joined Search; Spotify and SoundCloud joined Media; npm, PyPI and Hugging Face joined Dev; Mojeek joined News; and **Neshan** joined Maps (`neshan.org/maps/search/…`) with its own icon
- **The direction button is a search button now** — the magnifying glass beside the box runs the search, and the box still works out for itself which way to read
- **Dock tooltips are everywhere** — every dock tool, and the Settings gear below the divider, shows the styled name bubble instead of the browser's own
- **The dock no longer jumps** — the scrollbar's width is reserved ahead of time, so the icons stay centred and stay under your cursor when the list starts to scroll
- **The download table links its files** — every release's notes now point straight at the installers

### New in 1.4.9
- **Search got organised** — engines are grouped (Search, Community, Media, Learn, News, Maps, Dev), each button carries its own drawn logo, and three more joined: Bing, Google Images and Stack Overflow
- **The search box knows which way you read** — type Persian or Arabic and a direction button appears on the left; type Latin and it sits on the right. Press it to override the direction the text is shown in
- **The calculator keypad now matches the Windows layout** — ÷ sits on the 789 row, × on 456, − on 123, and the bottom row reads `^ 0 . =`. The equals key draws its real glyph (it was blank before)
- **You can type into the calculator** — digits, operators, brackets and `^` all work straight from the keyboard, `Enter` evaluates, `Backspace` deletes, `c` or `Delete` clears, and `Escape` still closes the panel
- **The calculator refuses nonsense** — a key that would build something unparseable is ignored, so `1**3++++--` can no longer be typed. Chained signs like `3--+6` still work
- **Folders are no longer dock shortcuts** — dropping a folder, or adding one, puts it in the **Folders** tool. That panel is now yours: add, remove and drag to reorder, and the `+ Folder` button is gone from Settings
- **Notes have a real right-click menu** — Edit title, Copy note, and Delete note (which keeps one empty note so the panel is never left blank)
- **Hover text is readable again** — the note menu used to turn its label the same colour as its background
- **Scrollbars are back, and themed** — thin rounded bars that take their colour from the active theme, instead of being hidden
- **Dock apps got tooltips** — a proper styled bubble with the app name, drawn by the panel rather than the browser's default `title`
- **Apps added to the dock now sit on a button background** like every other tool
- **The first row in the More panel is no longer clipped** when you hover it
- **The hour box is back in the timer**, and its Set button is an icon
- **A centring pass** — the World Clock delete button, the calculator operators and the widget manager rows all line up now. The row buttons read remove, move, toggle, and moving a widget is an arrow icon instead of the word "More"
- **Password minimum length is 4**
### New in 1.4.8
- **Num Convert behaves like a text tool now** — the input is a resizable area that keeps your line breaks, and the three result boxes are all exactly the same size and show every line you typed
- **Units panel layout** — the number box gets its own full-width row, the category and unit pickers sit underneath it 50/50, and the number box loses those up/down arrows
- **Calculator keypad is drawn, not typed** — digits stay plain text, and clear, backspace, brackets, ÷ × − + ^ = are now real icons. The result box draws its operators the same way
- **The calculator understands chained signs** — `3--+6` gives `9`, `3*-2` gives `-6`, `2^-3` gives `0.125`
- **Text tools take a proper text box** — resizable input that keeps line breaks, a matching result box, and the "using what you typed" line is gone
- **Note titles have a real editor** — double-click a tab, or right-click it and choose **Edit title**, and a text box opens in its place. Editing the label in place kept losing the caret to the row being redrawn
- **Generate panel** now lists the time in 12-hour form too, under the 24-hour one
- **Two more search engines**: Reddit and Brave
- **The widget on/off control is a proper eye icon** now, centred in its button
### New in 1.4.7
- **Units panel rebuilt** — a number box and a category dropdown, and it lists **every** unit in that category at once. Nine groups: length, mass, volume, data sizes (bit → PB), speed, temperature, frequency, energy and pressure. `1 MB` shows `1,048,576 B`, `1,024 KB` and `8,388,608 BIT` side by side, and any row copies on click
- **Calculator is a real calculator** — the text box is gone, replaced by a keypad: digits, `+ − × ÷`, powers, parentheses, backspace, clear and `=`. Click the result to copy it
- **Text tools get a result box** — the chosen format is shown in a box at the bottom with a copy button instead of being pasted over your clipboard, and the button you picked stays highlighted
- **Password generator rebuilt** — switch **A–Z**, **a–z**, **0–9** and symbols on and off individually, the slider now goes up to **64** characters, and a strength bar tells you how many bits you actually got. Characters are picked without modulo bias, so no symbol is favoured
- **Note titles are editable for real** — double-clicking a tab now actually opens the editor. Two things used to fight it: the click redrew the tab row just before the double-click landed, and a pending save could wipe the row mid-edit
- **Settings rows tidied up** — the "+ App" / "+ Folder" buttons get a real plus icon with matching padding, and the widget drag handle is no longer heavy-handed
- **New tool: Num Convert** — type or paste any text and it shows the same text with just the digits rewritten, in English, Farsi or Arabic. Letters, punctuation, signs and separators are left exactly as they were, and each row copies with one click, with a Clear button to start over
- **Generate panel preview is centred** vertically instead of sitting on the top line
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
