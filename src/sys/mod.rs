#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use self::windows::*;

#[cfg(not(windows))]
mod unix;
#[cfg(not(windows))]
pub use self::unix::*;

/// One look at the eyedropper's pointer: the color under it, where it is on the screen, and
/// the magnified picture around it as a data URI. The picture is empty when the screen would
/// not give it up, and then the page keeps the one it already has.
#[derive(Clone)]
pub struct Look {
    pub hex: String,
    pub x: i32,
    pub y: i32,
    pub png: String,
}
