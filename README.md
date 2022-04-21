# ttycarousel - Console animations for Rust

This crate provides simple spinner animations for console, to ensure your users
do not get bored and do not think that the program is dead.

<img
src="https://raw.githubusercontent.com/divi255/ttycarousel/main/demo.gif" />

Crate: <https://crates.io/crates/ttycarousel>

## Sync programs

Add to Cargo.toml:

```toml
[dependencies]
ttycarousel = { version = "*", features = ["sync"] }
```

### Simple

```rust
use std::time::Duration;

ttycarousel::spawn0("working");
std::thread::sleep(Duration::from_secs(2));
ttycarousel::stop();
println!("work completed!");
```

### With options

```rust
use std::time::Duration;

ttycarousel::spawn(
    "working",
    ttycarousel::Options::new()
        .speed(50)
        .color(ttycarousel::Color::Yellow)
        .bold(),
);
std::thread::sleep(Duration::from_secs(2));
ttycarousel::stop();
```

## Async (Tokio)

Add to Cargo.toml:

```toml
[dependencies]
ttycarousel = { version = "*", features = ["tokio1"] }
```

Async example:

```rust
use std::time::Duration;

async fn task1() {
    ttycarousel::tokio1::spawn(
        "working",
        ttycarousel::Options::new()
            .speed(50)
            .color(ttycarousel::Color::Yellow)
            .bold(),
    ).await;
    //ttycarousel::tokio1::spawn0("working").await; // with defaults
    tokio::time::sleep(Duration::from_secs(2)).await;
    ttycarousel::tokio1::stop().await;
    println!("work completed!");
}
```

## P.S.

Yep, I had nothing to do.
