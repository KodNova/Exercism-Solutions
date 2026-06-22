use std::fmt;

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_min = minutes + (hours * 60);
        let new_minutes = total_min.rem_euclid(60);
        let new_hours = (total_min - new_minutes) / 60;

        Clock {
            hours: new_hours.rem_euclid(24),
            minutes: new_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }

    pub fn add_hours(&self, hours: i32) -> Self {
        Clock::new(self.hours + hours, self.minutes)
    }
}
