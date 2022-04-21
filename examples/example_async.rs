use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let to_sleep = Duration::from_secs(2);
    ttycarousel::tokio1::spawn0(" Preparing").await;
    sleep(to_sleep).await;
    ttycarousel::tokio1::spawn0(" Doing some task").await;
    sleep(to_sleep).await;
    ttycarousel::tokio1::stop_with("OK!").await;
    ttycarousel::tokio1::spawn0(" Doing one more task").await;
    sleep(to_sleep).await;
    ttycarousel::tokio1::stop_with("OK!").await;
    ttycarousel::tokio1::spawn0(" Doing another task").await;
    sleep(to_sleep).await;
    ttycarousel::tokio1::stop_with("OK!").await;
    println!(" Work completed!");
}
