//! # ttycarousel - Console animations for Rust
//! 
//! This crate provides a simple carousel animation for console, to ensure your
//! users do not get bored and do not think that the program is dead.
//! 
//! <img
//! src="https://raw.githubusercontent.com/divi255/ttycarousel/main/demo.gif" />
//! 
//! Crate: <https://crates.io/crates/ttycarousel>
//! 
//! ## Sync programs
//! 
//! Add to Cargo.toml:
//! 
//! ```toml
//! [dependencies]
//! ttycarousel = { version = "*", features = ["sync"] }
//! ```
//! 
//! ### Simple
//! 
//! ```rust
//! use std::time::Duration;
//! 
//! ttycarousel::spawn0("working");
//! std::thread::sleep(Duration::from_secs(2));
//! ttycarousel::stop();
//! println!("work completed!");
//! ```
//! 
//! ### With options
//! 
//! ```rust
//! use std::time::Duration;
//! 
//! ttycarousel::spawn(
//!     "working",
//!     ttycarousel::Options::new()
//!         .speed(50)
//!         .color(ttycarousel::Color::Yellow)
//!         .bold(),
//! );
//! std::thread::sleep(Duration::from_secs(2));
//! ttycarousel::stop();
//! ```
//! 
//! ## Async (Tokio)
//! 
//! Add to Cargo.toml:
//! 
//! ```toml
//! [dependencies]
//! ttycarousel = { version = "*", features = ["tokio1"] }
//! ```
//! 
//! Async example:
//! 
//! ```rust
//! use std::time::Duration;
//! 
//! async fn task1() {
//!     ttycarousel::tokio1::spawn(
//!         "working",
//!         ttycarousel::Options::new()
//!             .speed(50)
//!             .color(ttycarousel::Color::Yellow)
//!             .bold(),
//!     );
//!     //ttycarousel::tokio1::spawn0("working"); // with defaults
//!     tokio::time::sleep(Duration::from_secs(2)).await;
//!     ttycarousel::tokio1::stop().await;
//!     println!("work completed!");
//! }
//! ```
//! 
//! ## P.S.
//! 
//! Yep, I had nothing to do.
use std::time::Duration;

#[cfg(any(feature = "sync", feature = "tokio1"))]
mod carousel;
#[cfg(feature = "sync")]
pub mod sync;
#[cfg(feature = "tokio1")]
pub mod tokio1;

#[cfg(feature = "sync")]
pub use sync::{spawn, spawn0, stop};

const DEFAULT_DELAY: Duration = Duration::from_millis(50);

#[cfg(any(feature = "sync", feature = "tokio1"))]
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
    #[cfg(any(feature = "sync", feature = "tokio1"))]
    fn as_escape(self) -> [u8; 5] {
        [0x1b, b'[', b'3', self as u8 + 48, b'm']
    }
}

#[cfg(any(feature = "sync", feature = "tokio1"))]
#[inline]
fn cleanup() {
    if atty::is(atty::Stream::Stdout) {
        println!("\x1b[D ");
    }
}
