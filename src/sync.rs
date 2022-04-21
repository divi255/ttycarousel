use crate::{carousel::TaskResult, Options};
use std::fmt;
use std::io::{self, Write};
use std::sync::atomic;
use std::sync::Mutex;
use std::thread;

lazy_static::lazy_static! {
    static ref TASK: Mutex<Option<thread::JoinHandle<TaskResult>>> = <_>::default();
}

static ACTIVE: atomic::AtomicBool = atomic::AtomicBool::new(false);

#[inline]
fn print_flush(buf: &[u8]) {
    let mut stdout = io::stdout();
    let _r = stdout.write_all(buf);
    let _r = stdout.flush();
}

#[inline]
pub fn spawn0(prompt: impl fmt::Display) {
    spawn(prompt, Options::default());
}

/// # Panics
///
/// Will panic if the mutex is poisoned
pub fn spawn(prompt: impl fmt::Display, opts: Options) {
    if ACTIVE.load(atomic::Ordering::SeqCst) {
        stop();
    }
    ACTIVE.store(true, atomic::Ordering::SeqCst);
    if atty::is(atty::Stream::Stdout) {
        print_flush(format!("{}  ", prompt).as_bytes());
        let task = thread::spawn(move || rotate(opts));
        TASK.lock().unwrap().replace(task);
    } else {
        print_flush(format!("{}...  ", prompt).as_bytes());
        let _r = io::stdout().flush();
    }
}

#[inline]
pub fn stop() {
    stop_carousel(Some(""));
}

#[inline]
pub fn stop_with(res: impl fmt::Display) {
    stop_carousel(Some(res));
}

#[inline]
pub fn stop_clear() {
    stop_carousel(None::<&str>);
}

/// # Panics
///
/// Will panic if the mutex is poisoned
pub fn stop_carousel(res: Option<impl fmt::Display>) {
    ACTIVE.store(false, atomic::Ordering::SeqCst);
    if let Some(task) = TASK.lock().unwrap().take() {
        let _r = task.join();
    }
    if let Some(s) = res {
        crate::carousel::cleanup(s);
    } else {
        print_flush(crate::carousel::CLREOL);
    }
}

fn rotate(opts: Options) -> TaskResult {
    let mut c = crate::carousel::Carousel::new(opts);
    let mut stdout = io::stdout();
    while ACTIVE.load(atomic::Ordering::Relaxed) {
        stdout.write_all(c.rotate())?;
        stdout.flush()?;
        thread::sleep(opts.delay);
    }
    Ok(())
}
