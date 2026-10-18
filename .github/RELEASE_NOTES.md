![Right Panel](https://raw.githubusercontent.com/WhoisNeon/right-panel/main/docs/preview-laptop.png)

### Downloads
Everything below is a direct link to a file published with **v1.4.28**.

| Platform | Get it |
|---|---|
| **Windows** (recommended) | **[RightPanel-Setup.exe](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.28/RightPanel-Setup.exe)** installer · or [RightPanel-portable.exe](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.28/RightPanel-portable.exe) / [RightPanel-windows-x64.zip](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.28/RightPanel-windows-x64.zip) |
| **macOS** *(beta)* | [RightPanel-macos-arm64.dmg](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.28/RightPanel-macos-arm64.dmg) (Apple Silicon) · [RightPanel-macos-x64.dmg](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.28/RightPanel-macos-x64.dmg) (Intel). Right-click → Open the first time. Zips too: [arm64](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.28/RightPanel-macos-arm64.zip) · [x64](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.28/RightPanel-macos-x64.zip) |
| **Linux** *(beta)* | [RightPanel-linux-amd64.deb](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.28/RightPanel-linux-amd64.deb) · [RightPanel-linux-x64.tar.gz](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.28/RightPanel-linux-x64.tar.gz) (X11 + WebKitGTK 4.1) |

Every release keeps its own files — see [all releases](https://github.com/WhoisNeon/right-panel/releases).
For what changed in every earlier version, see the [full changelog](https://github.com/WhoisNeon/right-panel/blob/main/docs/CHANGELOG.md).

### New in 1.4.28
- **The panel stops going dead** — with the invisible part of the strip let clicks through, a stale measurement could leave the window refusing every click while still on screen, and quitting the app was the only way out. Input is now given back whenever the panel opens, and given back again on its own if the pointer sits outside for five seconds
- **The Network tool shows its values** — the rows were reading the second character of each value, so an address came out as `2` and a country as `e`. All nine rows fit without a scrollbar now, and the cross on an added time zone is centred
- **Memory reads to one decimal** — `16 GB (15 GB usable)` becomes `16 GB (15.8 GB usable)`, the same figure the Performance meters already gave
- **The eyedropper's swatch stays put** — instead of chasing the pointer it sits in one of two fixed spots by the panel edge and steps aside when the pointer comes near. It is bigger, rounder, and the hex sits on a chip beside it
- **The calculator never slides its value** — the `translateX` that kept the last digits in view was being measured into the panel's click boundary, which is what left the strip swallowing clicks. The value now shrinks to a floor of 16px and, if it still cannot fit, the box shows its end
- **The calendar keeps its boxes on screen** — a six-week month used to push the date fields off the bottom of the panel. The days are now the only part that gives way; the two date cards and the year/month/day boxes stay where they are
- **The documentation is in order** — the release notes hold only this release, the full history moved to [`docs/CHANGELOG.md`](https://github.com/WhoisNeon/right-panel/blob/main/docs/CHANGELOG.md), the README was brought up to date, and [`AGENTS.md`](https://github.com/WhoisNeon/right-panel/blob/main/AGENTS.md) describes the project for coding agents