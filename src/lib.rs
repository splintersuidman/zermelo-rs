extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate reqwest;

mod schedule;
mod appointment;

pub use schedule::*;
pub use appointment::*;
