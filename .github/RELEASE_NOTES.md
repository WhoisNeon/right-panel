![Right Panel](https://raw.githubusercontent.com/WhoisNeon/right-panel/main/docs/preview-laptop.png)

### Downloads
Everything below is a direct link to a file published with **v1.4.31**.

| Platform | Get it |
|---|---|
| **Windows** (recommended) | **[RightPanel-Setup.exe](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.31/RightPanel-Setup.exe)** installer · or [RightPanel-portable.exe](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.31/RightPanel-portable.exe) / [RightPanel-windows-x64.zip](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.31/RightPanel-windows-x64.zip) |
| **macOS** *(beta)* | [RightPanel-macos-arm64.dmg](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.31/RightPanel-macos-arm64.dmg) (Apple Silicon) · [RightPanel-macos-x64.dmg](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.31/RightPanel-macos-x64.dmg) (Intel). Right-click → Open the first time. Zips too: [arm64](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.31/RightPanel-macos-arm64.zip) · [x64](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.31/RightPanel-macos-x64.zip) |
| **Linux** *(beta)* | [RightPanel-linux-amd64.deb](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.31/RightPanel-linux-amd64.deb) · [RightPanel-linux-x64.tar.gz](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.31/RightPanel-linux-x64.tar.gz) (X11 + WebKitGTK 4.1) |

Every release keeps its own files — see [all releases](https://github.com/WhoisNeon/right-panel/releases).
For what changed in every earlier version, see the [full changelog](https://github.com/WhoisNeon/right-panel/blob/main/docs/CHANGELOG.md).

### New in 1.4.31
- **A key on its own can open the panel** — the Keybinds capture treated Ctrl, Shift, Alt and Win as "still choosing" and threw them away, so only a combination could be bound and the tray's *Hold key to open* was the only way to get Ctrl on its own. A lone modifier now binds when it is released, and Ctrl + Q still binds as a pair
- **Hold key to open is gone from the tray** — it offered four fixed choices and could not express a combination, so `Ctrl + Shift + Space` showed up there as `Ctrl`. Opening on a key is the panel's own setting (*Settings → Keybinds*) and is not duplicated in the tray
- **Reset to default is in the tray** — it opens Settings on the General tab and arms the button there, so it still takes a second tap rather than wiping the dock and the themes from one stray menu click
- **The Settings panel changes height with its tabs** — every page carried a floor set by the shortest one, which is why a tall page and a short page sat in the same box. Each page now takes its own height. Switching tabs no longer walks the panel up the screen either: the frame loop was re-deriving the top from a spring that had not settled
- **The eyedropper shows a crosshair** — the full-screen window that takes the picking click had no cursor of its own, and Windows reads a class with none as the app-start cursor, which is the hourglass. It carries the crosshair of a screen ruler now
- **The Network tool's open-domain button is gone** — the Domain row is still there and still copies like every other row
- **One spelling for color** — the British spelling is gone from the panel, the README and these notes