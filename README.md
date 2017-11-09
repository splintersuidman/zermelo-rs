# zermelo-rs [![Build Status](https://travis-ci.org/splintah/zermelo-rs.svg?branch=master)](https://travis-ci.org/splintah/zermelo-rs)
A Rust library that retrieves a schedule from Zermelo.

# Docs
Add this crate in your Cargo.toml:
```toml
[dependencies]
zermelo = { git = "https://github.com/splintah/zermelo-rs" }
```

Now, in your main.rs, replacing the strings with your own values:
```rust
extern crate zermelo;

fn main() {
    // Get schedule using:
    let schedule = Schedule::new("school", "code").unwrap();
    println!("{}", schedule.access_token);
    // or, when you have got an access token already:
    let schedule = Schedule::with_access_token("school", "access token");

    schedule.get_appointments(start, end).unwrap();
    for appointment in schedule.appointments {
        println!("{}", appointment);
    }
}
```
