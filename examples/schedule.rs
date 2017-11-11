extern crate chrono;
extern crate zermelo;

use chrono::prelude::*;

fn main() {
    // Replace these with your own properties.
    // If access_token is set, code does not have to be set.
    let school = String::from("school");
    let code = String::from("code");
    let access_token = String::from("access token");

    let mut schedule: zermelo::Schedule;

    if !code.is_empty() {
        // If we have got an authentication code:
        schedule = zermelo::Schedule::new(school, code).unwrap();
        println!("{}", schedule.access_token);
    } else if !access_token.is_empty() {
        // else if you have got an access token already:
        schedule = zermelo::Schedule::with_access_token(school, access_token);
    } else {
        std::process::exit(1);
    }

    // Get today's times.
    let dt = Local::now();
    let start = dt.with_hour(0)
        .unwrap()
        .with_minute(0)
        .unwrap()
        .with_second(0)
        .unwrap()
        .timestamp();
    let end = dt.with_hour(23)
        .unwrap()
        .with_minute(59)
        .unwrap()
        .with_second(59)
        .unwrap()
        .timestamp();

    // Get appointments between start and end.
    schedule.get_appointments(start, end).unwrap();
    // Print appointments.
    for appointment in schedule.appointments {
        println!("{:?}", appointment);
    }
}
