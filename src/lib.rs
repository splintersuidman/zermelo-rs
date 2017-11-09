#[macro_use]
extern crate serde_json;

mod schedule;
mod appointment;

pub use schedule::*;
pub use appointment::*;
