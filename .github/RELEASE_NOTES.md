![Right Panel](https://raw.githubusercontent.com/WhoisNeon/right-panel/main/docs/preview-laptop.png)

### Downloads
Everything below is a direct link to a file published with **v1.4.30**.

| Platform | Get it |
|---|---|
| **Windows** (recommended) | **[RightPanel-Setup.exe](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.30/RightPanel-Setup.exe)** installer · or [RightPanel-portable.exe](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.30/RightPanel-portable.exe) / [RightPanel-windows-x64.zip](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.30/RightPanel-windows-x64.zip) |
| **macOS** *(beta)* | [RightPanel-macos-arm64.dmg](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.30/RightPanel-macos-arm64.dmg) (Apple Silicon) · [RightPanel-macos-x64.dmg](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.30/RightPanel-macos-x64.dmg) (Intel). Right-click → Open the first time. Zips too: [arm64](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.30/RightPanel-macos-arm64.zip) · [x64](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.30/RightPanel-macos-x64.zip) |
| **Linux** *(beta)* | [RightPanel-linux-amd64.deb](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.30/RightPanel-linux-amd64.deb) · [RightPanel-linux-x64.tar.gz](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.30/RightPanel-linux-x64.tar.gz) (X11 + WebKitGTK 4.1) |

Every release keeps its own files — see [all releases](https://github.com/WhoisNeon/right-panel/releases).
For what changed in every earlier version, see the [full changelog](https://github.com/WhoisNeon/right-panel/blob/main/docs/CHANGELOG.md).

### New in 1.4.30
- **Recent colours are the colours again** — they are stored without their `#`, which is what the colour tool works in, and that was put straight into a CSS background. The rule was silently dropped and every swatch came out white or black. The hash is put back when it is drawn
- **Picking a colour does not click through** — two faults. The window that takes the picking click is registered under a fixed class name, and registering an existing class is refused; that was treated as a reason to give up, so after the first pick there was nothing in the way at all. And a layered window at zero alpha is not reliably hit-tested, so clicks went through it as if it were not there. It now registers once and reuses the class, and sits at 1/255 — still invisible, and it holds the click
- **ASN and Domain in the Network tool** — the network the address belongs to, and the domain it is reached at, with a button beside the domain that opens it
- **The Settings panel fits its content** — every page was pinned to the height of the tallest one, so Theme sat in a tall box with most of it empty. Each page now takes its own height, over a floor set by the shortest, and scrolls when there is more than the screen has room for
- **The timer cannot be edited while it runs** — the scroll, the arrow keys, the digits and the presets are all refused mid-countdown, the cursor stops offering itself, and the display says to pause it first