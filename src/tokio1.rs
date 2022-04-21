use crate::{Options, TaskResult};
use std::fmt;
use std::future::Future;
use std::sync::atomic;
use std::sync::Mutex;
use tokio::io::{self, AsyncWriteExt};
use tokio::task::JoinHandle;

lazy_static::lazy_static! {
    static ref TASK: Mutex<Option<JoinHandle<TaskResult>>> = <_>::default();
}

static ACTIVE: atomic::AtomicBool = atomic::AtomicBool::new(false);

#[inline]
pub fn spawn0(prompt: impl fmt::Display) -> impl Future<Output = ()> {
    spawn(prompt, Options::default())
}

/// # Panics
///
/// Will panic if the mutex is poisoned
pub async fn spawn(prompt: impl fmt::Display, opts: Options) {
    if ACTIVE.load(atomic::Ordering::SeqCst) {
        stop().await;
    }
    ACTIVE.store(true, atomic::Ordering::SeqCst);
    if atty::is(atty::Stream::Stdout) {
        print!("{}  ", prompt);
        let task = tokio::spawn(rotate(opts));
        TASK.lock().unwrap().replace(task);
    } else {
        let mut stdout = io::stdout();
        let _r = stdout.write_all(format!("{}... ", prompt).as_bytes()).await;
        let _r = stdout.flush().await;
    }
}

#[inline]
pub fn stop() -> impl Future<Output = ()> {
    stop_with("")
}

/// # Panics
///
/// Will panic if the mutex is poisoned
pub async fn stop_with(res: &str) {
    ACTIVE.store(false, atomic::Ordering::SeqCst);
    let task = TASK.lock().unwrap().take();
    if let Some(fut) = task {
        fut.abort();
        // await for task because on high speeds it might fail to be stopped
        let _r = fut.await;
    }
    crate::cleanup(res);
}

async fn rotate(opts: Options) -> TaskResult {
    let mut c = crate::carousel::Carousel::new(opts);
    let mut stdout = io::stdout();
    loop {
        stdout.write_all(c.rotate()).await?;
        stdout.flush().await?;
        tokio::time::sleep(opts.delay).await;
    }
}
