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

pub struct Appointment {
    pub appointment_instance: Option<i64>,
    pub id: Option<i64>,

    // When does this appointment take place?
    pub start: Option<i64>,
    pub end: Option<i64>,
    pub start_time_slot: Option<i64>,
    pub end_time_slot: Option<i64>,

    // What is this appointment about?
    pub subjects: Vec<String>,
    // `type` is a reserved keyword.
    pub appointment_type: Option<AppointmentType>,
    pub remark: Option<String>,

    // Where does this appointment take place?
    pub locations: Vec<String>,
    // TODO: pub locations_of_branch: ?,

    // Who is participating in this appointment?
    pub teachers: Vec<String>,
    pub groups: Vec<String>,
    // TODO: pub grops_in_department: ?,

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
    pub fn from_json_map(lesson: &Value) -> Appointment {
        let lesson = lesson.as_object().unwrap();

        // Parse subjects.
        let mut subjects: Vec<String> = Vec::new();
        if let Some(subjects_json) = lesson.get("subjects").unwrap_or(&Value::Null).as_array() {
            for subject in subjects_json {
                if let Some(subject) = subject.as_str() {
                    subjects.push(subject.to_owned());
                }
            }
        }

        // Parse teachers.
        let mut teachers: Vec<String> = Vec::new();
        if let Some(teachers_json) = lesson.get("teachers").unwrap_or(&Value::Null).as_array() {
            for teacher in teachers_json {
                if let Some(teacher) = teacher.as_str() {
                    teachers.push(teacher.to_owned());
                }
            }
        }

        // Parse groups.
        let mut groups: Vec<String> = Vec::new();
        if let Some(groups_json) = lesson.get("groups").unwrap_or(&Value::Null).as_array() {
            for group in groups_json {
                if let Some(group) = group.as_str() {
                    groups.push(group.to_owned());
                }
            }
        }

        // Parse locations.
        let mut locations: Vec<String> = Vec::new();
        if let Some(locations_json) = lesson.get("locations").unwrap_or(&Value::Null).as_array() {
            for location in locations_json {
                if let Some(location) = location.as_str() {
                    locations.push(location.to_owned());
                }
            }
        }

        // Parse AppointmentType from str.
        let mut appointment_type: Option<AppointmentType> = None;
        if let Some(appointment_type_str) = lesson.get("type").unwrap_or(&Value::Null).as_str() {
            appointment_type = AppointmentType::from_str(appointment_type_str);
        };

        // Parse remark string.
        let remark: Option<String>;
        let remark_str = lesson.get("remark").unwrap_or(&Value::Null).as_str();
        if let Some(remark_str) = remark_str {
            remark = Some(remark_str.to_owned());
        } else {
            remark = None;
        }

        // Parse change_description string.
        let change_description: Option<String>;
        let change_description_str = lesson
            .get("changeDescription")
            .unwrap_or(&Value::Null)
            .as_str();
        if let Some(change_description_str) = change_description_str {
            change_description = Some(change_description_str.to_owned());
        } else {
            change_description = None;
        }

        // Parse branch string.
        let branch: Option<String>;
        let branch_str = lesson.get("branch").unwrap_or(&Value::Null).as_str();
        if let Some(branch_str) = branch_str {
            branch = Some(branch_str.to_owned());
        } else {
            branch = None;
        }

        // Parse i64's and bools (easy to parse).
        let start_time_slot = lesson.get("startTimeSlot").unwrap_or(&Value::Null).as_i64();
        let appointment_instance = lesson
            .get("appointmentInstance")
            .unwrap_or(&Value::Null)
            .as_i64();
        let branch_of_school = lesson
            .get("branchOfSchool")
            .unwrap_or(&Value::Null)
            .as_i64();
        let id = lesson.get("id").unwrap_or(&Value::Null).as_i64();
        let start = lesson.get("start").unwrap_or(&Value::Null).as_i64();
        let end = lesson.get("end").unwrap_or(&Value::Null).as_i64();
        let end_time_slot = lesson.get("endTimeSlot").unwrap_or(&Value::Null).as_i64();
        let created = lesson.get("created").unwrap_or(&Value::Null).as_i64();
        let last_modified = lesson.get("lastModified").unwrap_or(&Value::Null).as_i64();
        let valid = lesson.get("valid").unwrap_or(&Value::Null).as_bool();
        let hidden = lesson.get("hidden").unwrap_or(&Value::Null).as_bool();
        let cancelled = lesson.get("cancelled").unwrap_or(&Value::Null).as_bool();
        let modified = lesson.get("modified").unwrap_or(&Value::Null).as_bool();
        let moved = lesson.get("moved").unwrap_or(&Value::Null).as_bool();
        let new = lesson.get("new").unwrap_or(&Value::Null).as_bool();

        // Push appointment to self.appointments.
        Appointment {
            appointment_instance,
            id,
            start,
            end,
            start_time_slot,
            end_time_slot,
            subjects,
            appointment_type,
            remark,
            locations,
            teachers,
            groups,
            created,
            last_modified,
            valid,
            hidden,
            cancelled,
            modified,
            moved,
            new,
            change_description,
            branch_of_school,
            branch,
        }
    }
}

impl fmt::Debug for Appointment {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "#{}\nsubjects: {:?}\nlocations: {:?}\nteachers: {:?}\ngroups: {:?}\n",
            self.start_time_slot.unwrap_or(0),
            self.subjects.as_slice().join(", "),
            self.locations.as_slice().join(", "),
            self.teachers.as_slice().join(", "),
            self.groups.as_slice().join(", ")
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
