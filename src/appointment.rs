extern crate serde_json;

use std::fmt;
use self::serde_json::Value;

pub enum AppointmentType {
    Unknown,
    Lesson,
    Exam,
    Activity,
    Choice,
    Talk,
    Other,
}

impl AppointmentType {
    pub fn from_str(t: &str) -> Option<Self> {
        match t {
            "unknown" => Some(AppointmentType::Unknown),
            "lesson" => Some(AppointmentType::Lesson),
            "exam" => Some(AppointmentType::Exam),
            "activity" => Some(AppointmentType::Activity),
            "choice" => Some(AppointmentType::Choice),
            "talk" => Some(AppointmentType::Talk),
            "other" => Some(AppointmentType::Other),
            _ => None,
        }
    }
}

#[derive(Deserialize)]
pub struct Appointment {
    pub appointment_instance: Option<i64>,
    pub id: Option<i64>,

    // When does this appointment take place?
    pub start: Option<i64>,
    pub end: Option<i64>,
    pub start_time_slot: Option<i64>,
    pub end_time_slot: Option<i64>,

    // What is this appointment about?
    pub subjects: Option<Vec<String>>,
    // `type` is a reserved keyword.
    pub appointment_type: Option<String>,
    pub remark: Option<String>,

    // Where does this appointment take place?
    pub locations: Option<Vec<String>>,
    // TODO: pub locations_of_branch: ?,

    // Who is participating in this appointment?
    pub teachers: Option<Vec<String>>,
    pub groups: Option<Vec<String>>,
    // TODO: pub groups_in_department: ?,

    // What is the status of the version of this appointment?
    pub created: Option<i64>,
    pub last_modified: Option<i64>,
    pub valid: Option<bool>,
    pub hidden: Option<bool>,
    pub cancelled: Option<bool>,
    pub modified: Option<bool>,
    pub moved: Option<bool>,
    pub new: Option<bool>,
    pub change_description: Option<String>,

    // Where does this appointment belong to?
    pub branch_of_school: Option<i64>,
    pub branch: Option<String>,
}

impl Appointment {
    pub fn from_json_map(lesson: Value) -> Result<Appointment, serde_json::Error> {
        let appointment: Appointment = serde_json::from_value(lesson)?;
        Ok(appointment)
    }
}

impl fmt::Debug for Appointment {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "#{}\nsubjects: {:?}\nlocations: {:?}\nteachers: {:?}\ngroups: {:?}\n",
            self.start_time_slot.unwrap_or(-1),
            self.subjects
                .clone()
                .unwrap_or(vec![])
                .as_slice()
                .join(", "),
            self.locations
                .clone()
                .unwrap_or(vec![])
                .as_slice()
                .join(", "),
            self.teachers
                .clone()
                .unwrap_or(vec![])
                .as_slice()
                .join(", "),
            self.groups.clone().unwrap_or(vec![]).as_slice().join(", ")
        )
    }
}

#[cfg(test)]
mod tests {
    use appointment::Appointment;

    #[test]
    fn appointment_from_json_map() {
        let map = json!({
            "start": 1510185600,
            "end": 1510271999,
            "startTimeSlot": 0,
            "endTimeSlot": 9,
            "subjects": ["netl"]
        });

        let appointment = Appointment::from_json_map(&map);
        assert_eq!(appointment.start, Some(1510185600));
        assert_eq!(appointment.end, Some(1510271999));
        assert_eq!(appointment.start_time_slot, Some(0));
        assert_eq!(appointment.end_time_slot, Some(9));
        assert_eq!(appointment.subjects, vec![String::from("netl")]);
    }
}
