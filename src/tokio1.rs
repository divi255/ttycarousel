use crate::{carousel::TaskResult, Options};
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
async fn print_flush(buf: &[u8]) {
    let mut stdout = io::stdout();
    let _r = stdout.write_all(buf).await;
    let _r = stdout.flush().await;
}

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
        print_flush(format!("{}  ", prompt).as_bytes()).await;
        let task = tokio::spawn(rotate(opts));
        TASK.lock().unwrap().replace(task);
    } else {
        print_flush(format!("{}... ", prompt).as_bytes()).await;
    }
}

#[inline]
pub fn stop() -> impl Future<Output = ()> {
    stop_carousel(Some(""))
}

#[inline]
pub fn stop_with(res: impl fmt::Display) -> impl Future<Output = ()> {
    stop_carousel(Some(res))
}

#[inline]
pub fn stop_clear() -> impl Future<Output = ()> {
    stop_carousel(None::<&str>)
}

async fn stop_carousel(res: Option<impl fmt::Display>) {
    ACTIVE.store(false, atomic::Ordering::SeqCst);
    let task = TASK.lock().unwrap().take();
    if let Some(fut) = task {
        fut.abort();
        // await for task because on high speeds it might fail to be stopped
        let _r = fut.await;
    }
    if let Some(s) = res {
        crate::carousel::cleanup(s);
    } else {
        print_flush(crate::carousel::CLREOL).await;
    }
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
