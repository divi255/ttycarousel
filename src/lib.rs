use std::time::Duration;

mod carousel;
#[cfg(feature = "sync")]
pub mod sync;
#[cfg(feature = "tokio1")]
pub mod tokio1;

#[cfg(feature = "sync")]
pub use sync::{spawn, spawn0, stop};

const DEFAULT_DELAY: Duration = Duration::from_millis(50);

type TaskResult = Result<(), std::io::Error>;

#[derive(Copy, Clone)]
pub struct Options {
    reverse: bool,
    delay: Duration,
    color: Option<Color>,
    bold: bool,
}

impl Default for Options {
    #[inline]
    fn default() -> Self {
        Self {
            reverse: false,
            delay: DEFAULT_DELAY,
            color: None,
            bold: false,
        }
    }
}

impl Options {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
    #[inline]
    pub fn reverse(mut self) -> Self {
        self.reverse = true;
        self
    }
    #[inline]
    pub fn speed(mut self, speed: u64) -> Self {
        self.delay = Duration::from_millis(speed);
        self
    }
    #[inline]
    pub fn color(mut self, color: Color) -> Self {
        self.color.replace(color);
        self
    }
    #[inline]
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }
}

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Red = 1,
    Green = 2,
    Yellow = 3,
    Blue = 4,
    Magenta = 5,
    Cyan = 6,
    White = 7,
}

impl Color {
    #[allow(dead_code)]
    fn as_escape(self) -> [u8; 5] {
        [0x1b, b'[', b'3', self as u8 + 48, b'm']
    }
}

#[allow(dead_code)]
#[inline]
fn cleanup() {
    if atty::is(atty::Stream::Stdout) {
        println!("\x1b[D ");
    }
}
