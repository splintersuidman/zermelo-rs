use reqwest;
use serde_json;
use appointment::*;
use std::io::Read;
use std::error::Error;

/// This struct represents the schedule, containing [Appointment](./struct.Appointment.html)s.
pub struct Schedule {
    /// The school id used in the URL.
    /// For the school id of 'example', this URL will be: `https://example.zportal.nl/`.
    pub school: String,
    /// The access token obtained from the API, used to obtain appointments.
    pub access_token: String,
    /// A vector of the appointments.
    pub appointments: Vec<Appointment>,
}

impl Schedule {
    /// Create a new `Schedule` from an authorization code (only once usable) and a school identifier.
    /// This will get the access token from the API.
    /// Returns a `Schedule` or an error.
    pub fn new(school: String, code: String) -> Result<Self, String> {
        let url = format!("https://{}.zportal.nl/api/v3/oauth/token", school);
        // Remove spaces from code.
        let code = code.replace(" ", "");
        let post_data = [("grant_type", "autorization_code"), ("code", code.as_str())];

        // Send request.
        let response = reqwest::Client::new()
            .post(url.as_str())
            .form(&post_data)
            .send();
        let mut response = match response {
            Ok(res) => res,
            Err(e) => return Err(format!("could not make request: {}", e.description())),
        };

        // Check whether response code equals "200 OK".
        if response.status().as_u16() != 200 {
            return Err("response code is not 200".to_owned());
        }

        // Parse response as JSON.
        let json: AuthenticationResponse = match response.json() {
            Ok(j) => j,
            Err(e) => return Err(format!("could not parse response as JSON: {}", e.description())),
        };

        let access_token = json.access_token;

        Ok(Schedule {
            school,
            access_token,
            appointments: Vec::new(),
        })
    }

    /// Create a new `Schedule` when an access token has been obtained already.
    /// This cannot fail, so this will not return a `Result`.
    pub fn with_access_token(school: String, access_token: String) -> Self {
        Schedule {
            school,
            access_token: access_token.to_string(),
            appointments: Vec::new(),
        }
    }

    /// Get the appointments between `start` and `end` from the API, and set them to `self.appointments`.
    /// Returns a reference to itself, or an error.
    pub fn get_appointments(&mut self, start: i64, end: i64) -> Result<&Self, String> {
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
            Err(e) => return Err(format!("could not make request: {}", e.description())),
        };

        // Check whether response code equals "200 OK".
        if response.status().as_u16() != 200 {
            return Err("response code is not 200".to_owned());
        }

        // Read body to string.
        let mut body = String::new();
        match response.read_to_string(&mut body) {
            Ok(res) => res,
            Err(e) => {
                return Err(format!(
                    "could read response to string: {}",
                    e.description()
                ))
            }
        };

        // Replace camelCase index with snake_case index, so we can deserialize it easier.
        let body = body.replace("appointmentInstance", "appointment_instance")
            .replace("startTimeSlot", "start_time_slot")
            .replace("endTimeSlot", "end_time_slot")
            .replace("type", "appointment_type")
            .replace("lastModified", "lastModified")
            .replace("changeDescription", "change_description")
            .replace("branchOfSchool", "branch_of_school");

        println!("{}", body);

        let response: AppointmentsResponse = match serde_json::from_str(body.as_str()) {
            Ok(res) => res,
            Err(e) => {
                return Err(format!(
                    "could parse JSON from response: {}",
                    e.description()
                ))
            }
        };

        self.appointments = response.response.data;

        // Sort appointments by start time.
        self.appointments
            .sort_unstable_by_key(|k| k.start.unwrap_or(0));

        Ok(self)
    }
}

#[derive(Deserialize)]
struct AuthenticationResponse {
    access_token: String,
}

#[derive(Deserialize)]
struct AppointmentsResponse {
    response: AppointmentsResponseResponse,
}

#[derive(Deserialize)]
// Why, Zermelo, would you wrap everything in a "response" map?
struct AppointmentsResponseResponse {
    data: Vec<Appointment>,
}

#[cfg(test)]
mod tests {
    use serde_json;
    use schedule::*;

    #[test]
    fn parse_request() {
        // Data example from https://zermelo.atlassian.net/wiki/spaces/DEV/pages/22577247/Example+Retrieving+a+Schedule.
        let json = r#"{
            "response": {
                "status": 200,
                "message": "",
                "startRow": 0,
                "endRow": 27,
                "totalRows": 27,
                "data": [
                    {
                        "id": 5,
                        "start": 42364236,
                        "end": 436234523,
                        "startTimeSlot": 1,
                        "endTimeSlot": 1,
                        "subjects": ["ne"],
                        "teachers": ["KRO"],
                        "groups": ["v1a"],
                        "locations": ["M92"],
                        "type": "lesson",
                        "remark": "Take care to bring your books",
                        "valid": true,
                        "cancelled": false,
                        "modified": true,
                        "moved": false,
                        "new": false,
                        "changeDescription": "The location has been changed from M13 to M92"
                    }
                ]
            }
        }"#;

        let json = json.replace("appointmentInstance", "appointment_instance")
            .replace("startTimeSlot", "start_time_slot")
            .replace("endTimeSlot", "end_time_slot")
            .replace("type", "appointment_type")
            .replace("lastModified", "lastModified")
            .replace("changeDescription", "change_description")
            .replace("branchOfSchool", "branch_of_school");

        let response: AppointmentsResponse = serde_json::from_str(json.as_str()).unwrap();
        let appointment = &response.response.data[0];
        assert_eq!(appointment.id, Some(5));
        assert_eq!(appointment.start, Some(42364236));
        assert_eq!(appointment.start_time_slot, Some(1));
        assert_eq!(appointment.subjects, Some(vec![String::from("ne")]));
        assert_eq!(appointment.appointment_type, Some(String::from("lesson")));
        assert_eq!(appointment.cancelled, Some(false));
    }
}
