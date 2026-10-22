#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use self::windows::*;

#[cfg(not(windows))]
mod unix;
#[cfg(not(windows))]
pub use self::unix::*;

/// One look at the eyedropper's pointer: the color under it and where it is on the screen.
#[derive(Clone)]
pub struct Look {
    pub hex: String,
    pub x: i32,
    pub y: i32,
}
