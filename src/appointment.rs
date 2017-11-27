/// You can use this enum to parse `Appointment.appointment_type`.
/// This is done to make matching easier.
///
/// ```no_run
/// # use zermelo::AppointmentType;
/// let t = AppointmentType::parse("exam"); // => Some(AppointmentType::Exam)
/// let n = AppointmentType::parse("abc"); // => None
/// ```
pub enum AppointmentType {
    /// `unknown` in Zermelo's API.
    Unknown,
    /// `lesson` in Zermelo's API.
    Lesson,
    /// `exam` in Zermelo's API.
    Exam,
    /// `activity` in Zermelo's API.
    Activity,
    /// `choice` in Zermelo's API.
    Choice,
    /// `talk` in Zermelo's API.
    Talk,
    /// `other` in Zermelo's API.
    Other,
}

impl AppointmentType {
    /// Parse appointment type from `&str`.
    /// Returns an optional `AppointmentType`.
    pub fn parse(t: &str) -> Option<Self> {
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

/// This struct represents an appointment in the schedule.
/// It should (more or less) match [Zermelo's appointment specification](https://zermelo.atlassian.net/wiki/spaces/DEV/pages/15860217/Appointment).
///
/// This struct does not have an implementation with methods like `Appointment::new()`, because is
/// is deserialised by serde.
///
/// **Note**: instead of camelCase, as used by Zermelo in their JSON, this struct uses `snake_case`, as common in Rust programs.
/// Example: `startTimeSlot` becomes `start_time_slot`.
///
/// **Note**: `appointment_type` is called `type` in Zermelo's API, but `type` is a reserved keyword in Rust.
///
/// See [Zermelo's appointment documentation](https://zermelo.atlassian.net/wiki/spaces/DEV/pages/15860217/Appointment)
/// for a full explanation of this struct's members.
///
/// On this page a part of Zermelo's explanation can be found.
#[derive(Deserialize)]
pub struct Appointment {
    /// The id of the appointment instance this appointment version belongs to.
    pub appointment_instance: Option<i64>,
    /// The internal id of this version of the appointment.
    pub id: Option<i64>,

    /// UTC Unix time of the start of this appointment (the number of seconds elapsed since 1 January 1970).
    pub start: Option<i64>,
    /// UTC Unix time of the end of this appointment (the number of seconds elapsed since 1 January 1970).
    pub end: Option<i64>,
    /// The numerical designation of the time slot during which this appointment starts. Usual values are between 1 and 9.
    pub start_time_slot: Option<i64>,
    /// The numerical designation of the time slot during which this appointment ends. Usual values are between 1 and 9.
    pub end_time_slot: Option<i64>,

    /// The (human readable) subjects names or abbreviations this appointment is about.
    pub subjects: Option<Vec<String>>,
    /// The type of this appointment. See [AppointmentType](./enum.AppointmentType.html) for more information.
    /// **Note**: this member is called `type` in Zermelo's API, but `type` is a reserved keyword in Rust.
    pub appointment_type: Option<String>,
    /// Remark for this appointment. For example: "Don't forget your books".
    pub remark: Option<String>,

    /// The names of the locations (classrooms) where this appointment will take place.
    pub locations: Option<Vec<String>>,
    // The identifiers of the locations (classrooms) where this appointment will take place.
    // TODO: pub locations_of_branch: ?,

    /// The (three letter) codes/abbreviations of the teachers participating in this appointment.
    pub teachers: Option<Vec<String>>,
    /// The names of the student groups participating in this appointment.
    pub groups: Option<Vec<String>>,
    // A list of the actual groups participating in this appointment.
    // TODO: pub groups_in_department: ?,

    /// The UTC Unix Time of when this appointment version was created. Useful to show a chronological view of the history of this appointment.
    pub created: Option<i64>,
    /// Unix time (UTC) at which any fields of this version of the appointment were modified or when this version of the appointment was created.
    pub last_modified: Option<i64>,
    /// True if this appointment is part of the most up-to-date schedule.
    pub valid: Option<bool>,
    /// True if this version of the appointment was hidden and should not be shown to users.
    pub hidden: Option<bool>,
    /// True if this appointment has been cancelled.
    pub cancelled: Option<bool>,
    /// True if anything at all was changed in this version of the appointment.
    pub modified: Option<bool>,
    /// True if the start or end time or the location of this appointment were modified.
    pub moved: Option<bool>,
    /// True if this appointment has been added and was not originally scheduled.
    pub new: Option<bool>,
    /// A textual description of the change to be shown to the user.
    pub change_description: Option<String>,

    /// Internal ID of the branch of the schoolInSchoolyear (Dutch: vestiging van school in schooljaar) this appointment belongs to.
    pub branch_of_school: Option<i64>,
    /// Convenience field providing the branch code of the branchOfSchool.
    pub branch: Option<String>,
}

impl ::std::fmt::Debug for Appointment {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(
            f,
            "#{}\nsubjects: {:?}\nlocations: {:?}\nteachers: {:?}\ngroups: {:?}\n",
            self.start_time_slot.unwrap_or(-1),
            self.subjects
                .clone()
                .unwrap_or_default()
                .as_slice()
                .join(", "),
            self.locations
                .clone()
                .unwrap_or_default()
                .as_slice()
                .join(", "),
            self.teachers
                .clone()
                .unwrap_or_default()
                .as_slice()
                .join(", "),
            self.groups.clone().unwrap_or_default().as_slice().join(", ")
        )
    }
}

#[cfg(test)]
mod tests {
    use appointment::Appointment;
    use serde_json;

    #[test]
    fn appointment_from_json() {
        let json = r#"{
            "start": 1510185600,
            "end": 1510271999,
            "start_time_slot": 0,
            "end_time_slot": 9,
            "subjects": [
                "netl"
            ]
        }"#;

        let appointment: Appointment = serde_json::from_str(json).unwrap();
        assert_eq!(appointment.start, Some(1510185600));
        assert_eq!(appointment.end, Some(1510271999));
        assert_eq!(appointment.start_time_slot, Some(0));
        assert_eq!(appointment.end_time_slot, Some(9));
        assert_eq!(appointment.subjects, Some(vec![String::from("netl")]));
    }
}
