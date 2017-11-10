extern crate zermelo;
extern crate chrono;

use chrono::prelude::*;

fn main() {
    let school = String::from("cgu");
    let code = String::from("");
    let access_token = String::from("kc6i2ete286rednaqj3gp8890f");

    let mut schedule: zermelo::Schedule;

    if code.len() > 0 {
        // If we have got an authentication code:
        schedule = zermelo::Schedule::new(school, code).unwrap();
        println!("{}", schedule.access_token);
    } else if access_token.len() > 0 {
        // else if you have got an access token already:
        schedule = zermelo::Schedule::with_access_token(school, access_token);
    } else {
        std::process::exit(1);
    }

    // Get today's times.
    let dt = Local::now();
    let start = dt.with_hour(0).unwrap()
        .with_minute(0).unwrap()
        .with_second(0).unwrap()
        .timestamp();
    let end = dt.with_hour(23).unwrap()
        .with_minute(59).unwrap()
        .with_second(59).unwrap()
        .timestamp();

    // Get appointments between start and end.
    schedule.get_appointments(start, end).unwrap();
    // Print appointments.
    for appointment in schedule.appointments {
        println!("{:?}", appointment);
    }
}
