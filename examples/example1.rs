use std::thread::sleep;
use std::time::Duration;

fn main() {
    let to_sleep = Duration::from_secs(2);
    ttycarousel::spawn0(" Doing some task");
    sleep(to_sleep);
    ttycarousel::stop();
    ttycarousel::spawn0(" Doing another task");
    sleep(to_sleep);
    ttycarousel::stop();
    ttycarousel::spawn0(" Doing one more task");
    sleep(to_sleep);
    ttycarousel::stop();
    println!(" Work completed!");
}
