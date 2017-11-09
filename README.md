# zermelo-rs [![Build Status](https://travis-ci.org/splintah/zermelo-rs.svg?branch=master)](https://travis-ci.org/splintah/zermelo-rs)
A Rust library that retrieves a schedule from Zermelo.

# Docs
Add this crate in your Cargo.toml:
```toml
[dependencies]
zermelo = { git = "https://github.com/splintah/zermelo-rs" }
chrono = "0.4"
```

Now, in your main.rs, replacing the strings with your own values:
```rust
extern crate zermelo;
extern crate chrono;

fn main() {
    // Get schedule using:
    let schedule = zermelo::Schedule::new("school", "code").unwrap();
    println!("{}", schedule.access_token);
    // or, when you have got an access token already:
    let schedule = zermelo::Schedule::with_access_token("school", "access token");

    let dt = Local::now();
    let start = dt.with_hour(0).unwrap()
        .with_minute(0).unwrap()
        .with_second(0).unwrap()
        .timestamp();
    let end = dt.with_hour(23).unwrap()
        .with_minute(59).unwrap()
        .with_second(59).unwrap()
        .timestamp();

    schedule.get_appointments(start, end).unwrap();
    for appointment in schedule.appointments {
        println!("{}", appointment);
    }
}
```
