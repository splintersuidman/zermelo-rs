//! This is a crate that can be used to retrieve a schedule from [Zermelo](https://www.zermelo.nl/).
//! The goal of this crate is to be easy to use, whilst also being powerful.
//!
//! # Examples
//!
//! ## Obtaining an access token
//! ```no_run
//! let school = "example";
//! let code = "123456789012";
//!
//! let schedule = zermelo::Schedule::new(school, code).unwrap();
//! println!("Your access token is: {}", schedule.access_token);
//! ```
//! You have now successfully obtained an access token, so you can continue with [Retrieving a schedule](#retrieving-a-schedule).
//!
//! ## Creating schedule with an access token
//! ```no_run
//! let school = "example";
//! let access_token = "abcdefghijklmnopqrstuvwxyz";
//!
//! let schedule = zermelo::Schedule::with_access_token(school, access_token);
//! ```
//! You can continue with [Retrieving a schedule](#retrieving-a-schedule).
//!
//! ## Retrieving a schedule
//! This example assumes you have got a mutable variable named `schedule`, obtained with one of the above methods.
//! Before running this with the properties found in the Zermelo portal, set `start` and `end` to something else.
//! I recommend using [chrono](https://crates.io/crates/chrono) to get today's time.
//!
//! ```no_run
//! # extern crate zermelo;
//! # let school = "example";
//! # let access_token = "abcdefghijklmnopqrstuvwxyz";
//! # let mut schedule = zermelo::Schedule::with_access_token(school, access_token);
//! // These should be set to something else.
//! let start: i64 = 0;
//! let end: i64 = 10;
//!
//! // Get schedule.
//! schedule.get_appointments(start, end).unwrap();
//!
//! // Print schedule.
//! for appointment in schedule.appointments {
//!     println!("{:?}", appointment);
//! }
//! ```
//! See [Schedule](./struct.Schedule.html) for the Schedule struct and its members.

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate reqwest;

mod schedule;
mod appointment;

pub use schedule::Schedule;
pub use appointment::{Appointment, AppointmentType};
