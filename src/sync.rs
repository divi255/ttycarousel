use crate::{Options, TaskResult};
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
pub fn spawn0(prompt: impl fmt::Display) {
    spawn(prompt, Options::default());
}

/// # Panics
///
/// Will panic if the mutex is poisoned
pub fn spawn(prompt: impl fmt::Display, opts: Options) {
    if atty::is(atty::Stream::Stdout) {
        print!("{}  ", prompt);
        ACTIVE.store(true, atomic::Ordering::SeqCst);
        let task = thread::spawn(move || rotate(opts));
        TASK.lock().unwrap().replace(task);
    } else {
        print!("{}...", prompt);
    }
}

/// # Panics
///
/// Will panic if the mutex is poisoned
pub fn stop() {
    if let Some(task) = TASK.lock().unwrap().take() {
        ACTIVE.store(false, atomic::Ordering::SeqCst);
        let _r = task.join();
    }
    crate::cleanup();
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
