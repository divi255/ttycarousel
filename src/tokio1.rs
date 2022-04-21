use crate::{Options, TaskResult};
use std::fmt;
use std::sync::Mutex;
use tokio::io::{self, AsyncWriteExt};
use tokio::task::JoinHandle;

lazy_static::lazy_static! {
    static ref TASK: Mutex<Option<JoinHandle<TaskResult>>> = <_>::default();
}

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
        let task = tokio::spawn(rotate(opts));
        TASK.lock().unwrap().replace(task);
    } else {
        print!("{}...", prompt);
    }
}

/// # Panics
///
/// Will panic if the mutex is poisoned
pub async fn stop() {
    let task = TASK.lock().unwrap().take();
    if let Some(fut) = task {
        fut.abort();
        // await for task because on high speeds it might fail to be stopped
        let _r = fut.await;
    }
    crate::cleanup();
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
