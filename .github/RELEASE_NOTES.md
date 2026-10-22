![Right Panel](https://raw.githubusercontent.com/WhoisNeon/right-panel/main/docs/preview-laptop.png)

### Downloads
Everything below is a direct link to a file published with **v1.4.33**.

| Platform | Get it |
|---|---|
| **Windows** (recommended) | **[RightPanel-Setup.exe](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.33/RightPanel-Setup.exe)** installer · or [RightPanel-portable.exe](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.33/RightPanel-portable.exe) / [RightPanel-windows-x64.zip](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.33/RightPanel-windows-x64.zip) |
| **macOS** *(beta)* | [RightPanel-macos-arm64.dmg](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.33/RightPanel-macos-arm64.dmg) (Apple Silicon) · [RightPanel-macos-x64.dmg](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.33/RightPanel-macos-x64.dmg) (Intel). Right-click → Open the first time. Zips too: [arm64](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.33/RightPanel-macos-arm64.zip) · [x64](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.33/RightPanel-macos-x64.zip) |
| **Linux** *(beta)* | [RightPanel-linux-amd64.deb](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.33/RightPanel-linux-amd64.deb) · [RightPanel-linux-x64.tar.gz](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.33/RightPanel-linux-x64.tar.gz) (X11 + WebKitGTK 4.1) |

Every release keeps its own files — see [all releases](https://github.com/WhoisNeon/right-panel/releases).
For what changed in every earlier version, see the [full changelog](https://github.com/WhoisNeon/right-panel/blob/main/docs/CHANGELOG.md).

### New in 1.4.33
- **The eyedropper magnifies what is under the pointer** — the previous cursor only marked the spot. Now the panel sends back the 15 × 15 screen pixels around the pointer, each drawn ten times, as a 150px loupe beside the arrow: the square in the middle is exactly the pixel that will be picked, and the color and its hex sit along the bottom. Escape gives the screen back and the loupe goes with it
- **One combination, one owner** — the same key on two rows used to open two tools in whatever order the map happened to be walked, and a tool sharing the hold key never opened at all. Binding a key that is already taken now hands it over: the previous row lets go and the panel says so
- **The Keybinds tab is the height of the Widgets tab** — it measured its own content, so it came up shorter than the tab beside it. The Widgets page is measured once and Keybinds is held to that same box
