use std::thread::sleep;
use std::time::Duration;

fn main() {
    let to_sleep = Duration::from_secs(2);
    ttycarousel::spawn0(" Preparing");
    sleep(to_sleep);
    ttycarousel::spawn0(" Running rm -rf /");
    sleep(to_sleep);
    ttycarousel::stop_clear();
    println!(" Just kidding! ;-)");
    ttycarousel::spawn0(" Doing some task");
    sleep(to_sleep);
    ttycarousel::stop_with("OK!");
    ttycarousel::spawn0(" Doing one more task");
    sleep(to_sleep);
    ttycarousel::stop_with("OK!");
    ttycarousel::spawn0(" Doing another task");
    sleep(to_sleep);
    ttycarousel::stop_with("OK!");
    println!(" Work completed!");
}
