![Right Panel](https://raw.githubusercontent.com/WhoisNeon/right-panel/main/docs/preview-laptop.png)

### Downloads
Everything below is a direct link to a file published with **v1.4.29**.

| Platform | Get it |
|---|---|
| **Windows** (recommended) | **[RightPanel-Setup.exe](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.29/RightPanel-Setup.exe)** installer · or [RightPanel-portable.exe](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.29/RightPanel-portable.exe) / [RightPanel-windows-x64.zip](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.29/RightPanel-windows-x64.zip) |
| **macOS** *(beta)* | [RightPanel-macos-arm64.dmg](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.29/RightPanel-macos-arm64.dmg) (Apple Silicon) · [RightPanel-macos-x64.dmg](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.29/RightPanel-macos-x64.dmg) (Intel). Right-click → Open the first time. Zips too: [arm64](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.29/RightPanel-macos-arm64.zip) · [x64](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.29/RightPanel-macos-x64.zip) |
| **Linux** *(beta)* | [RightPanel-linux-amd64.deb](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.29/RightPanel-linux-amd64.deb) · [RightPanel-linux-x64.tar.gz](https://github.com/WhoisNeon/right-panel/releases/download/v1.4.29/RightPanel-linux-x64.tar.gz) (X11 + WebKitGTK 4.1) |

Every release keeps its own files — see [all releases](https://github.com/WhoisNeon/right-panel/releases).
For what changed in every earlier version, see the [full changelog](https://github.com/WhoisNeon/right-panel/blob/main/docs/CHANGELOG.md).

### New in 1.4.29
- **The eyedropper reads the colour that is really there** — it was reading the pixel through `GetPixel`, whose `COLORREF` did not come back in the documented order from the desktop's own device context, so red and blue came out swapped: a `#069021` green was picked as `#219006`. The pixel is now read through `GetDIBits`, whose 32bpp layout is documented and does not vary
- **Picking a colour no longer presses what is under the pointer** — while the grab is up, a transparent window the size of every screen sits in front of the desktop and takes the click. It draws nothing, so the colour being sampled is still the colour on screen, and it never takes focus. It is deliberately not a low-level mouse hook: a hook runs on every mouse event in the system and made the pointer feel wrong, which is why it was dropped last time
- **Memory is back** — it had gone from `16 GB (15.8 GB usable)` to a dash. The shared conversion was dividing kilobytes by 1024 three times, which is terabytes, so both figures rounded to zero. It takes bytes now, and the kilobyte one is converted first
- **The settings panel fits the screen and scrolls** — the pages were all forced to the height of the tallest one, which on a short screen is taller than the panel can ever be, so the Keybinds list had nowhere to scroll. The height is now capped to the room the panel has, and whatever does not fit scrolls inside the page
- **The eyedropper swatch has nothing painted over it** — the highlight ring that sat on top of the colour is gone