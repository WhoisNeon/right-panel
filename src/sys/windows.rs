//! Windows implementation: Win32 APIs for paste, keys, startup, icons, eyedropper…

use std::{
    ffi::c_void,
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicIsize, Ordering},
    thread,
    time::Duration,
};

use crate::util;
use windows_sys::Win32::{
    System::Com::CoTaskMemFree,
    Foundation::{CloseHandle, GetLastError, ERROR_ALREADY_EXISTS, HANDLE, HWND, POINT},
    Graphics::Gdi::{GetDC, GetPixel, ReleaseDC, BI_BITFIELDS},
    System::{
        DataExchange::{CloseClipboard, EmptyClipboard, GetClipboardData, GetClipboardSequenceNumber, OpenClipboard, SetClipboardData},
        Memory::{GlobalAlloc, GlobalLock, GlobalUnlock, GMEM_MOVEABLE},
        Ole::CF_DIB,
        Diagnostics::Debug::MessageBeep,
        Power::{SetThreadExecutionState, ES_CONTINUOUS, ES_DISPLAY_REQUIRED, ES_SYSTEM_REQUIRED},
        Registry::{RegDeleteKeyValueW, RegDeleteTreeW, RegGetValueW, RegSetKeyValueW, HKEY_CURRENT_USER, REG_SZ, RRF_RT_REG_SZ},
        Threading::CreateMutexW,
        Shutdown::LockWorkStation,
    },
    UI::Input::KeyboardAndMouse::{
        GetAsyncKeyState, SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, KEYEVENTF_UNICODE, VK_CONTROL, VK_D, VK_ESCAPE, VK_LBUTTON, VK_LWIN, VK_SHIFT, VK_SPACE,
        VK_MEDIA_NEXT_TRACK, VK_MEDIA_PLAY_PAUSE, VK_MEDIA_PREV_TRACK, VK_V, VK_VOLUME_DOWN, VK_VOLUME_MUTE, VK_VOLUME_UP,
    },
    UI::WindowsAndMessaging::{
        GetCursorPos, GetForegroundWindow, GetWindowTextW, PostMessageW, SetForegroundWindow, SetWindowPos, HWND_NOTOPMOST,
        HWND_TOPMOST, MB_ICONASTERISK, SC_MONITORPOWER, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, WM_SYSCOMMAND,
    },
};

pub const PLATFORM: &str = "windows";
const RUN_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
const RUN_NAME: &str = "RightPanel";

static PREV_FG: AtomicIsize = AtomicIsize::new(0);
static SELF_HWND: AtomicIsize = AtomicIsize::new(0);
/// the window the user last pinned, so unpinning hits the same one even after they switch away
static PINNED_HWND: AtomicIsize = AtomicIsize::new(0);

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}

pub fn data_dir() -> PathBuf {
    let base = PathBuf::from(std::env::var("APPDATA").unwrap_or_else(|_| ".".into()));
    let dir = base.join("RightPanel");
    // carry settings over from the old "Edge Dock" name
    // (copy, not move: the old folder can still be locked by WebView2)
    let old = base.join("EdgeDock");
    let _ = fs::create_dir_all(&dir);
    if old.join("settings.json").exists() && !dir.join("migrated").exists() {
        for f in ["settings.json", "notes.txt"] {
            let _ = fs::copy(old.join(f), dir.join(f));
        }
        let _ = fs::write(dir.join("migrated"), "");
        if reg_get("EdgeDock") {
            reg_del("EdgeDock");
            set_startup(true);
        }
    }
    dir
}

pub fn home() -> String {
    std::env::var("USERPROFILE").unwrap_or_default()
}

pub fn set_self_window(hwnd: isize) {
    SELF_HWND.store(hwnd, Ordering::Relaxed);
}

pub fn set_scale(_: f64) {}

/* ---------------- startup (HKCU\...\Run) ---------------- */
fn reg_get(name: &str) -> bool {
    let (k, v) = (wide(RUN_KEY), wide(name));
    unsafe { RegGetValueW(HKEY_CURRENT_USER, k.as_ptr(), v.as_ptr(), RRF_RT_REG_SZ, std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut()) == 0 }
}

fn reg_del(name: &str) {
    let (k, v) = (wide(RUN_KEY), wide(name));
    unsafe { RegDeleteKeyValueW(HKEY_CURRENT_USER, k.as_ptr(), v.as_ptr()) };
}

pub fn startup_enabled() -> bool {
    reg_get(RUN_NAME)
}

pub fn set_startup(on: bool) {
    if !on {
        return reg_del(RUN_NAME);
    }
    let (k, v) = (wide(RUN_KEY), wide(RUN_NAME));
    let exe = std::env::current_exe().map(|p| format!("\"{}\"", p.display())).unwrap_or_default();
    let data = wide(&exe);
    unsafe { RegSetKeyValueW(HKEY_CURRENT_USER, k.as_ptr(), v.as_ptr(), REG_SZ, data.as_ptr() as *const c_void, (data.len() * 2) as u32) };
}

/* ---------------- pointer ---------------- */
pub fn cursor_pos() -> Option<(i32, i32)> {
    let mut p = POINT { x: 0, y: 0 };
    (unsafe { GetCursorPos(&mut p) } != 0).then_some((p.x, p.y))
}

pub fn left_down() -> bool {
    unsafe { GetAsyncKeyState(VK_LBUTTON as i32) < 0 }
}

/// Is the key the user picked in Settings — 1 Ctrl, 2 Shift, 3 Space — currently held?
pub fn mod_held(which: u8) -> bool {
    let vk = match which {
        1 => VK_CONTROL,
        2 => VK_SHIFT,
        3 => VK_SPACE,
        _ => return true,
    };
    unsafe { GetAsyncKeyState(vk as i32) < 0 }
}

/// Remember the app the user was in, so paste / pin can target it.
pub fn remember_foreground() {
    let fg = unsafe { GetForegroundWindow() } as isize;
    if fg != SELF_HWND.load(Ordering::Relaxed) {
        PREV_FG.store(fg, Ordering::Relaxed);
    }
}

pub fn clip_seq() -> u64 {
    unsafe { GetClipboardSequenceNumber() as u64 }
}

/* ---------------- keyboard ---------------- */
fn key(vk: u16, up: bool) -> INPUT {
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: vk, wScan: 0, dwFlags: if up { KEYEVENTF_KEYUP } else { 0 }, time: 0, dwExtraInfo: 0 } },
    }
}

/// One Unicode character as a key press — the way to type text without touching the clipboard.
fn uni(scan: u16, up: bool) -> INPUT {
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 { ki: KEYBDINPUT { wVk: 0, wScan: scan, dwFlags: KEYEVENTF_UNICODE | if up { KEYEVENTF_KEYUP } else { 0 }, time: 0, dwExtraInfo: 0 } },
    }
}

fn send_combo(combo: &[u16]) {
    let mut inputs: Vec<INPUT> = combo.iter().map(|&k| key(k, false)).collect();
    inputs.extend(combo.iter().rev().map(|&k| key(k, true)));
    unsafe { SendInput(inputs.len() as u32, inputs.as_ptr(), std::mem::size_of::<INPUT>() as i32) };
}

/// Give focus back to the app the user was in, then press Ctrl+V.
pub fn paste_into_previous() {
    thread::spawn(|| {
        let prev = PREV_FG.load(Ordering::Relaxed);
        if prev != 0 {
            unsafe { SetForegroundWindow(prev as HWND) };
        }
        thread::sleep(Duration::from_millis(90));
        send_combo(&[VK_CONTROL, VK_V]);
    });
}

/// Type text into the app the user was in, one Unicode character at a time — no clipboard, no Ctrl+V.
/// Emoji outside the BMP arrive as a surrogate pair, which Windows recombines on the way in.
pub fn type_into_previous(text: &str) {
    let units: Vec<u16> = text.encode_utf16().collect();
    if units.is_empty() {
        return;
    }
    let prev = PREV_FG.load(Ordering::Relaxed);
    thread::spawn(move || {
        if prev != 0 {
            unsafe { SetForegroundWindow(prev as HWND) };
        }
        thread::sleep(Duration::from_millis(90));
        let mut inputs: Vec<INPUT> = Vec::with_capacity(units.len() * 2);
        for u in units {
            inputs.push(uni(u, false));
            inputs.push(uni(u, true));
        }
        unsafe { SendInput(inputs.len() as u32, inputs.as_ptr(), std::mem::size_of::<INPUT>() as i32) };
    });
}

/// Media / volume keys and Win+D, sent as real key presses.
pub fn press(name: &str) {
    let combo: &[u16] = match name {
        "play" => &[VK_MEDIA_PLAY_PAUSE],
        "next" => &[VK_MEDIA_NEXT_TRACK],
        "prev" => &[VK_MEDIA_PREV_TRACK],
        "mute" => &[VK_VOLUME_MUTE],
        "volup" => &[VK_VOLUME_UP],
        "voldown" => &[VK_VOLUME_DOWN],
        "desktop" => &[VK_LWIN, VK_D],
        _ => return,
    };
    send_combo(combo);
}

/* ---------------- system actions ---------------- */
fn win_title(hwnd: HWND) -> String {
    unsafe {
        let mut buf = [0u16; 120];
        let n = GetWindowTextW(hwnd, buf.as_mut_ptr(), buf.len() as i32).max(0) as usize;
        let title = String::from_utf16_lossy(&buf[..n]);
        if title.chars().count() > 28 { format!("{}…", title.chars().take(28).collect::<String>()) } else { title }
    }
}

/// Set "always on top" on the window the user was in before opening the panel.
///
/// Promoting that window also promoted it above the panel, which hid the panel itself — so the panel
/// is raised again straight after. The pinned hwnd is remembered, because the foreground window changes
/// as soon as the user goes back to what they were doing, and unpinning has to hit the same window.
pub fn set_topmost(on: bool) -> (bool, String) {
    let hwnd = if on { PREV_FG.load(Ordering::Relaxed) as HWND } else { PINNED_HWND.swap(0, Ordering::Relaxed) as HWND };
    if hwnd.is_null() {
        return (false, if on { "No window to pin".into() } else { "Nothing is pinned".into() });
    }
    unsafe {
        let title = win_title(hwnd);
        SetWindowPos(hwnd, if on { HWND_TOPMOST } else { HWND_NOTOPMOST }, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE);
        if on { PINNED_HWND.store(hwnd as isize, Ordering::Relaxed); }
        // keep the panel above the window it just pinned
        let self_hwnd = SELF_HWND.load(Ordering::Relaxed) as HWND;
        if !self_hwnd.is_null() { SetWindowPos(self_hwnd, HWND_TOPMOST, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE); }
        if on { (true, format!("{title} stays on top")) } else { (false, format!("Unpinned {title}")) }
    }
}

pub fn screen_off() {
    thread::spawn(|| {
        thread::sleep(Duration::from_millis(600)); // let the panel slide away first
        unsafe { PostMessageW(SELF_HWND.load(Ordering::Relaxed) as HWND, WM_SYSCOMMAND, SC_MONITORPOWER as usize, 2) };
    });
}

pub fn lock() {
    unsafe { LockWorkStation() };
}

pub fn beep() {
    unsafe { MessageBeep(MB_ICONASTERISK) };
}

/* ---------------- system monitor ----------------
   GetSystemTimes for CPU (busy = total minus idle) and GlobalMemoryStatusEx for RAM: two calls,
   no counter query. PDH's wildcard counters (GPU Engine, Network Interface) return an array per
   sample behind a flattened union, which is more machinery than a sparkline is worth and cannot be
   tested without a Windows box — those meters wait until there is a way to prove them.
   windows-sys 0.61 does not export GetSystemTimes, so that one is declared here: a FILETIME is two
   u32s, so the 100ns ticks read straight out of three u64s. */
use windows_sys::Win32::System::SystemInformation::{GetTickCount64, GlobalMemoryStatusEx, MEMORYSTATUSEX};

#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetSystemTimes(idle: *mut u64, kernel: *mut u64, user: *mut u64) -> i32;
}

pub struct Meters {
    last_idle: u64,
    last_total: u64,
    started: bool,
}

impl Meters {
    pub fn open() -> Option<Meters> {
        Some(Meters { last_idle: 0, last_total: 0, started: false })
    }

    /// Samples once. The first call has no previous reading to difference against, so it reports
    /// nothing rather than a nonsense spike.
    pub fn sample(&mut self) -> serde_json::Value {
        let (mut idle, mut kernel, mut user) = (0u64, 0u64, 0u64);
        let cpu = if unsafe { GetSystemTimes(&mut idle, &mut kernel, &mut user) } != 0 {
            let (i, t) = (idle, kernel + user);
            let ready = self.started && t > self.last_total;
            let pct = if ready {
                (((t - self.last_total) - i.saturating_sub(self.last_idle)) as f64 / (t - self.last_total) as f64) * 100.0
            } else {
                f64::NAN
            };
            self.last_idle = i;
            self.last_total = t;
            self.started = true;
            pct
        } else {
            f64::NAN
        };
        let mut mem = MEMORYSTATUSEX { dwLength: std::mem::size_of::<MEMORYSTATUSEX>() as u32, ..unsafe { std::mem::zeroed() } };
        let (ram, used, total) = if unsafe { GlobalMemoryStatusEx(&mut mem) } != 0 {
            (
                (mem.ullTotalPhys - mem.ullAvailPhys) as f64 / mem.ullTotalPhys.max(1) as f64 * 100.0,
                (mem.ullTotalPhys - mem.ullAvailPhys) / 1024 / 1024 / 1024,
                mem.ullTotalPhys / 1024 / 1024 / 1024,
            )
        } else {
            (0.0, 0, 0)
        };
        // how long the machine has been up, which is a free extra reading from the same call set
        let secs = unsafe { GetTickCount64() } / 1000;
        serde_json::json!({
            "cpu": if cpu.is_nan() { 0.0 } else { cpu },
            "ram": ram,
            "used": used,
            "total": total,
            "upDays": secs / 86400,
            "upHours": secs / 3600 % 24,
            "upMins": secs / 60 % 60,
        })
    }
}

/// ES_CONTINUOUS on the (long-lived) main thread keeps PC + display awake until turned off.
pub fn keep_awake(on: bool) {
    let f = if on { ES_CONTINUOUS | ES_SYSTEM_REQUIRED | ES_DISPLAY_REQUIRED } else { ES_CONTINUOUS };
    unsafe { SetThreadExecutionState(f) };
}

pub fn screenshot() {
    thread::spawn(|| {
        thread::sleep(Duration::from_millis(350));
        let _ = std::process::Command::new("explorer.exe").arg("ms-screenclip:").spawn();
    });
}

/// Screen eyedropper: waits for the next click, returns the pixel color.
pub fn pick_color(done: impl FnOnce(Option<String>) + Send + 'static) {
    thread::spawn(move || unsafe {
        while GetAsyncKeyState(VK_LBUTTON as i32) < 0 {
            thread::sleep(Duration::from_millis(10));
        }
        loop {
            thread::sleep(Duration::from_millis(10));
            if GetAsyncKeyState(VK_ESCAPE as i32) < 0 {
                return done(None);
            }
            if GetAsyncKeyState(VK_LBUTTON as i32) < 0 {
                let mut p = POINT { x: 0, y: 0 };
                GetCursorPos(&mut p);
                let dc = GetDC(std::ptr::null_mut());
                let c = GetPixel(dc, p.x, p.y);
                ReleaseDC(std::ptr::null_mut(), dc);
                return done(Some(format!("#{:02x}{:02x}{:02x}", c & 0xff, (c >> 8) & 0xff, (c >> 16) & 0xff)));
            }
        }
    });
}



use windows_sys::Win32::{
    Graphics::Gdi::{CreateCompatibleDC, DeleteDC, DeleteObject, GetDIBits, GetObjectW, BITMAP, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS},
    Storage::FileSystem::FILE_ATTRIBUTE_NORMAL,
    UI::{
        Controls::Dialogs::{GetOpenFileNameW, OFN_FILEMUSTEXIST, OFN_NODEREFERENCELINKS, OFN_PATHMUSTEXIST, OPENFILENAMEW},
        Controls::{ImageList_GetIcon, HIMAGELIST, ILD_TRANSPARENT},
        Shell::{
            SHBrowseForFolderW, SHGetFileInfoW, SHGetImageList, SHGetPathFromIDListW, ShellExecuteW, BIF_NEWDIALOGSTYLE, BIF_RETURNONLYFSDIRS, BROWSEINFOW,
            SHFILEINFOW, SHGFI_SYSICONINDEX, SHIL_EXTRALARGE,
        },
        WindowsAndMessaging::{DestroyIcon, GetIconInfo, PrivateExtractIconsW, HICON, ICONINFO, SW_SHOWNORMAL},
    },
};

/// Native "Open" dialog, starting in the Start Menu so shortcuts are easy to pick.
pub fn pick_file() -> Option<String> {
    let mut buf = vec![0u16; 1024];
    let filter: Vec<u16> = "Apps and shortcuts\0*.exe;*.lnk;*.url;*.bat;*.cmd\0All files\0*.*\0\0".encode_utf16().collect();
    let start = wide(r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs");
    let title = wide("Add app to Right Panel");
    unsafe {
        let mut ofn: OPENFILENAMEW = std::mem::zeroed();
        ofn.lStructSize = std::mem::size_of::<OPENFILENAMEW>() as u32;
        ofn.lpstrFilter = filter.as_ptr();
        ofn.lpstrFile = buf.as_mut_ptr();
        ofn.nMaxFile = buf.len() as u32;
        ofn.lpstrInitialDir = start.as_ptr();
        ofn.lpstrTitle = title.as_ptr();
        ofn.Flags = OFN_FILEMUSTEXIST | OFN_PATHMUSTEXIST | OFN_NODEREFERENCELINKS;
        if GetOpenFileNameW(&mut ofn) == 0 {
            return None;
        }
    }
    let len = buf.iter().position(|&c| c == 0).unwrap_or(0);
    Some(String::from_utf16_lossy(&buf[..len]))
}

pub fn launch(path: &str) {
    let (op, p) = (wide("open"), wide(path));
    let dir = Path::new(path).parent().filter(|d| d.is_dir()).map(|d| wide(&d.to_string_lossy()));
    let dir_ptr = dir.as_ref().map_or(std::ptr::null(), |d| d.as_ptr());
    unsafe { ShellExecuteW(std::ptr::null_mut(), op.as_ptr(), p.as_ptr(), std::ptr::null(), dir_ptr, SW_SHOWNORMAL) };
}

/// Icon of any file (exe gets a crisp 64px icon, shortcuts use the shell icon) as a PNG data URI.
pub fn icon_data_uri(path: &str) -> Option<String> {
    let p = wide(path);
    let mut icon: HICON = std::ptr::null_mut();
    unsafe {
        if path.to_lowercase().ends_with(".exe") {
            let mut id = 0u32;
            PrivateExtractIconsW(p.as_ptr(), 0, 64, 64, &mut icon, &mut id, 1, 0);
        }
        if icon.is_null() {
            // system image list index → 48px icon; ImageList_GetIcon without ILD_OVERLAYMASK has no shortcut arrow
            let mut info: SHFILEINFOW = std::mem::zeroed();
            SHGetFileInfoW(p.as_ptr(), FILE_ATTRIBUTE_NORMAL, &mut info, std::mem::size_of::<SHFILEINFOW>() as u32, SHGFI_SYSICONINDEX);
            const IID_IIMAGELIST: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x46eb5926_582e_4017_9fdf_e8998daa0950);
            let mut list: *mut core::ffi::c_void = std::ptr::null_mut();
            if SHGetImageList(SHIL_EXTRALARGE as i32, &IID_IIMAGELIST, &mut list) >= 0 && !list.is_null() {
                icon = ImageList_GetIcon(list as HIMAGELIST, info.iIcon, ILD_TRANSPARENT);
            }
        }
        if icon.is_null() {
            return None;
        }
        let out = icon_to_rgba(icon).map(|(w, h, px)| format!("data:image/png;base64,{}", util::base64(&util::png(w, h, &px))));
        DestroyIcon(icon);
        out
    }
}

unsafe fn icon_to_rgba(icon: HICON) -> Option<(u32, u32, Vec<u8>)> {
    unsafe {
        let mut ii: ICONINFO = std::mem::zeroed();
        if GetIconInfo(icon, &mut ii) == 0 || ii.hbmColor.is_null() {
            if !ii.hbmMask.is_null() { DeleteObject(ii.hbmMask); }
            return None;
        }
        let mut bm: BITMAP = std::mem::zeroed();
        GetObjectW(ii.hbmColor, std::mem::size_of::<BITMAP>() as i32, &mut bm as *mut _ as *mut _);
        let (w, h) = (bm.bmWidth as u32, bm.bmHeight as u32);
        let mut bi: BITMAPINFO = std::mem::zeroed();
        bi.bmiHeader = BITMAPINFOHEADER {
            biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: w as i32,
            biHeight: -(h as i32),
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB,
            ..std::mem::zeroed()
        };
        let mut px = vec![0u8; (w * h * 4) as usize];
        let dc = CreateCompatibleDC(std::ptr::null_mut());
        let ok = GetDIBits(dc, ii.hbmColor, 0, h, px.as_mut_ptr() as *mut _, &mut bi, DIB_RGB_COLORS);
        DeleteDC(dc);
        DeleteObject(ii.hbmColor);
        DeleteObject(ii.hbmMask);
        if ok == 0 {
            return None;
        }
        let has_alpha = px.chunks(4).any(|c| c[3] != 0);
        for c in px.chunks_mut(4) {
            c.swap(0, 2); // BGRA → RGBA
            if !has_alpha { c[3] = 255; }
        }
        Some((w, h, px))
    }
}


/* ---------------- single instance ---------------- */
/// Only one copy runs at a time; a second launch hands its job over through the inbox file.
pub fn single_instance() -> bool {
    let name = wide(r"Local\RightPanelSingleton");
    unsafe {
        let h = CreateMutexW(std::ptr::null(), 1, name.as_ptr());
        if h.is_null() {
            return true;
        }
        if GetLastError() == ERROR_ALREADY_EXISTS {
            CloseHandle(h);
            return false;
        }
        let _ = h; // the handle stays open for the life of the process, which is what holds the mutex
        true
    }
}

/* ---------------- Explorer right-click menu ---------------- */
// files, folders, drives, and the empty space inside a folder ("%V" = the folder itself)
const MENU_KEYS: [(&str, &str); 4] = [
    (r"Software\Classes\*\shell\RightPanel", "%1"),
    (r"Software\Classes\Directory\shell\RightPanel", "%1"),
    (r"Software\Classes\Drive\shell\RightPanel", "%1"),
    (r"Software\Classes\Directory\Background\shell\RightPanel", "%V"),
];

pub fn context_menu_enabled() -> bool {
    let k = wide(MENU_KEYS[0].0);
    unsafe { RegGetValueW(HKEY_CURRENT_USER, k.as_ptr(), std::ptr::null(), RRF_RT_REG_SZ, std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut()) == 0 }
}

/// Adds "Add to Right Panel" to the right-click menu of files and folders (per user, no admin).
pub fn set_context_menu(on: bool) {
    let exe = std::env::current_exe().map(|p| p.display().to_string()).unwrap_or_default();
    for (key, arg) in MENU_KEYS {
        let k = wide(key);
        unsafe {
            if !on {
                RegDeleteTreeW(HKEY_CURRENT_USER, k.as_ptr());
                continue;
            }
            let set = |sub: &Vec<u16>, name: &str, data: &str| {
                let (n, d) = (wide(name), wide(data));
                let name = if name.is_empty() { std::ptr::null() } else { n.as_ptr() };
                RegSetKeyValueW(HKEY_CURRENT_USER, sub.as_ptr(), name, REG_SZ, d.as_ptr() as *const c_void, (d.len() * 2) as u32);
            };
            set(&k, "", "Add to Right Panel");
            set(&k, "Icon", &exe);
            set(&k, "Position", "Top"); // otherwise it lands at the bottom of a crowded menu
            let cmd = wide(&format!(r"{key}\command"));
            set(&cmd, "", &format!("\"{exe}\" --add \"{arg}\""));
        }
    }
}

/* ---------------- clipboard images ---------------- */
/// Reads a bitmap off the clipboard as (width, height, RGBA).
pub fn clip_image() -> Option<(u32, u32, Vec<u8>)> {
    unsafe {
        if OpenClipboard(std::ptr::null_mut()) == 0 {
            return None;
        }
        let h = GetClipboardData(CF_DIB as u32);
        let out = (!h.is_null()).then(|| dib_to_rgba(h as *const u8)).flatten();
        CloseClipboard();
        out
    }
}

unsafe fn dib_to_rgba(dib: *const u8) -> Option<(u32, u32, Vec<u8>)> {
    unsafe {
        let head = &*(dib as *const BITMAPINFOHEADER);
        let (w, h) = (head.biWidth, head.biHeight);
        if w <= 0 || h == 0 || w > 10_000 || h.abs() > 10_000 || head.biBitCount < 24 {
            return None;
        }
        let (w, flip, rows) = (w as u32, h > 0, h.unsigned_abs());
        let bpp = (head.biBitCount / 8) as usize;
        let stride = ((w as usize * bpp) + 3) & !3;
        let masks = if head.biCompression == BI_BITFIELDS { 12 } else { 0 };
        let pixels = dib.add(head.biSize as usize + masks + head.biClrUsed as usize * 4);
        let mut out = vec![0u8; (w * rows * 4) as usize];
        for y in 0..rows as usize {
            let src_row = if flip { rows as usize - 1 - y } else { y };
            let src = pixels.add(src_row * stride);
            for x in 0..w as usize {
                let p = src.add(x * bpp);
                let i = (y * w as usize + x) * 4;
                out[i] = *p.add(2);
                out[i + 1] = *p.add(1);
                out[i + 2] = *p;
                out[i + 3] = if bpp == 4 { (*p.add(3)).max(1) } else { 255 };
            }
        }
        Some((w, rows, out))
    }
}

/// Puts an image back on the clipboard (as a 32-bit DIB, which every app understands).
pub fn set_clip_image(w: u32, h: u32, rgba: &[u8]) -> bool {
    let header = std::mem::size_of::<BITMAPINFOHEADER>();
    let mut buf = vec![0u8; header + (w * h * 4) as usize];
    unsafe {
        let head = &mut *(buf.as_mut_ptr() as *mut BITMAPINFOHEADER);
        head.biSize = header as u32;
        head.biWidth = w as i32;
        head.biHeight = h as i32; // positive: rows bottom-up
        head.biPlanes = 1;
        head.biBitCount = 32;
        head.biCompression = BI_RGB;
        head.biSizeImage = w * h * 4;
        for y in 0..h as usize {
            let src = (h as usize - 1 - y) * w as usize * 4;
            for x in 0..w as usize {
                let (s, d) = (src + x * 4, header + (y * w as usize + x) * 4);
                buf[d] = rgba[s + 2];
                buf[d + 1] = rgba[s + 1];
                buf[d + 2] = rgba[s];
                buf[d + 3] = rgba[s + 3];
            }
        }
        let mem = GlobalAlloc(GMEM_MOVEABLE, buf.len());
        if mem.is_null() {
            return false;
        }
        let dst = GlobalLock(mem);
        std::ptr::copy_nonoverlapping(buf.as_ptr(), dst as *mut u8, buf.len());
        GlobalUnlock(mem);
        if OpenClipboard(std::ptr::null_mut()) == 0 {
            return false;
        }
        EmptyClipboard();
        // on success the clipboard owns the block; on failure Windows frees it when we exit
        let ok = !SetClipboardData(CF_DIB as u32, mem as HANDLE).is_null();
        CloseClipboard();
        ok
    }
}

/// Folder picker (the classic browse dialog: small and dependency-free).
pub fn pick_folder() -> Option<String> {
    let title = wide("Add a folder to Right Panel");
    unsafe {
        let mut bi: BROWSEINFOW = std::mem::zeroed();
        bi.lpszTitle = title.as_ptr();
        bi.ulFlags = BIF_RETURNONLYFSDIRS | BIF_NEWDIALOGSTYLE;
        let list = SHBrowseForFolderW(&mut bi);
        if list.is_null() {
            return None;
        }
        let mut buf = [0u16; 520];
        let ok = SHGetPathFromIDListW(list, buf.as_mut_ptr()) != 0;
        CoTaskMemFree(list as *const c_void);
        let len = buf.iter().position(|&c| c == 0).unwrap_or(0);
        ok.then(|| String::from_utf16_lossy(&buf[..len]))
    }
}
