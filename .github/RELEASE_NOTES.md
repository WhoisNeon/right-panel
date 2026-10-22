![Right Panel](https://raw.githubusercontent.com/WhoisNeon/right-panel/main/docs/preview-laptop.png)

### Downloads
Everything below is a direct link to a file published with **v1.4.32**.

| Platform | Get it |
|---|---|
| **Windows** (recommended) | **[RightPanel-Setup.exe](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.32/RightPanel-Setup.exe)** installer · or [RightPanel-portable.exe](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.32/RightPanel-portable.exe) / [RightPanel-windows-x64.zip](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.32/RightPanel-windows-x64.zip) |
| **macOS** *(beta)* | [RightPanel-macos-arm64.dmg](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.32/RightPanel-macos-arm64.dmg) (Apple Silicon) · [RightPanel-macos-x64.dmg](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.32/RightPanel-macos-x64.dmg) (Intel). Right-click → Open the first time. Zips too: [arm64](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.32/RightPanel-macos-arm64.zip) · [x64](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.32/RightPanel-macos-x64.zip) |
| **Linux** *(beta)* | [RightPanel-linux-amd64.deb](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.32/RightPanel-linux-amd64.deb) · [RightPanel-linux-x64.tar.gz](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.32/RightPanel-linux-x64.tar.gz) (X11 + WebKitGTK 4.1) |

Every release keeps its own files — see [all releases](https://github.com/WhoisNeon/right-panel/releases).
For what changed in every earlier version, see the [full changelog](https://github.com/WhoisNeon/right-panel/blob/main/docs/CHANGELOG.md).

### New in 1.4.32
- **Settings now takes the height of the tab you are on** — every frame measured the panel while the previous frame's height cap was still on it, so it could only ever get shorter. Come back to the app on Keybinds, open Settings and switch to General, and the taller page was squeezed into the Keybinds box. The cap is cleared before measuring now, so the panel grows and shrinks with the active tab
- **A tall page gets the whole screen** — a page taller than the space left was pulled up by only half of what overflowed, which left the content area cut down on a small screen or with scaling turned up. It is pulled up by exactly what overflows, and never past the top margin
- **Keybinds has one more row** — the list shows six tools instead of five before it scrolls
- **Reset all on the Keybinds tab** — a small button in the header clears the hold key and every tool binding in one go. A capture that was still waiting for a key is dropped too, so the next keystroke does not write a binding back
- **The eyedropper has a magnifier cursor** — the crosshair is replaced by a small hollow ring with a dot in the middle, positioned so the dot marks exactly the pixel that will be picked. The center is no longer covered by the pointer
- **A clearer danger icon** — *Tap again to erase everything* now uses the circle alert icon instead of the octagon
