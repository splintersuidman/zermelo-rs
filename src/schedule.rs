extern crate reqwest;
extern crate serde_json;

use appointment::*;
use std::io::Read;
use std::error::Error;
use self::serde_json::Value;

pub struct Schedule {
    pub school: String,
    pub access_token: String,
    pub appointments: Vec<Appointment>,
}

impl Schedule {
    pub fn new(school: String, code: String) -> Result<Self, &'static str> {
        let url = format!("https://{}.zportal.nl/api/v3/oauth/token", school);
        let post_data = [("grant_type", "autorization_code"), ("code", code.as_str())];

        // Send request.
        let response = reqwest::Client::new()
            .post(url.as_str())
            .form(&post_data)
            .send();
        let mut response = match response {
            Ok(res) => res,
            Err(_) => return Err("could not make request"),
        };

        // Check whether response code equals "200 OK".
        if response.status().as_u16() != 200 {
            return Err("response code is not 200");
        }

        // Read response body to string.
        let mut body = String::new();
        match response.read_to_string(&mut body) {
            Ok(_) => {}
            Err(_) => return Err("could not read to string"),
        };

        // Parse body.
        let response: Value = match serde_json::from_str(body.as_str()) {
            Ok(v) => v,
            Err(_) => return Err("could not parse body as JSON"),
        };

        // Make sure response["access_token"] is a string.
        if !response["access_token"].is_string() {
            return Err("access token in response is not a string");
        }
        // We can safely unwrap here, because we checked whether it's a string.
        let access_token = response["access_token"].as_str().unwrap().to_string();

        Ok(Schedule {
            school,
            access_token,
            appointments: Vec::new(),
        })
    }

    pub fn with_access_token(school: String, access_token: String) -> Self {
        Schedule {
            school,
            access_token: access_token.to_string(),
            appointments: Vec::new(),
        }
    }

    pub fn get_appointments(&mut self, start: u64, end: u64) -> Result<&Self, String> {
        let url = format!(
            "https://{}.zportal.nl/api/v3/appointments?user=~me&start={}&end={}&access_token={}",
            self.school,
            start,
            end,
            self.access_token
        );

        // Make request.
        let mut response = match reqwest::get(url.as_str()) {
            Ok(res) => res,
            Err(e) => return Err(
                format!("could not make request: {}", e.description())
            ),
        };

        // Check whether response code equals "200 OK".
        if response.status().as_u16() != 200 {
            return Err("response code is not 200".to_owned());
        }

        // Read body to string.
        let mut body = String::new();
        match response.read_to_string(&mut body) {
            Ok(res) => res,
            Err(e) => return Err(
                format!("could read response to string: {}", e.description())
            ),
        };

        let response: Value = match serde_json::from_str(body.as_str()) {
            Ok(res) => res,
            Err(e) => return Err(
                format!("could parse JSON from response: {}", e.description())
            ),
        };

        // Get response from JSON, because why not wrap everything in "response"?
        let response = match response.get("response") {
            Some(res) => res,
            None => return Err("could not get response from JSON".to_owned()),
        };
        // Get lessons from data.
        let lessons = match response.get("data") {
            Some(l) => l,
            None => return Err("could not get data from response[\"response\"".to_owned())
        };
        let lessons = lessons.as_array().unwrap();

        // For every lesson, add it to self.appointments.
        for lesson in lessons {
            // Make sure lesson is an object.
            if !lesson.is_object() {
                return Err("lesson is not an object".to_owned());
            }
            let appointment = Appointment::from_json_map(lesson);
            self.appointments.push(appointment);
        }

        // Sort appointments by start time.
        self.appointments
            .sort_unstable_by_key(|k| k.start.unwrap_or(0));

        Ok(self)
    }
}
