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
        Clock {
            hours: hours,
            minutes: minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut hrs = 0;
        if minutes / 60 >= 1 {
            hrs = minutes / 60;
            self.add_hours(hrs);
        };

        let mut new_minutes = self.minutes + minutes;

        if new_minutes > 60 {
            new_minutes = 60;
        }

        Clock {
            hours: self.hours,
            minutes: new_minutes,
        }
    }

    pub fn add_hours(&self, hours: i32) -> Self {
        Clock {
            hours: self.hours + hours,
            minutes: self.minutes,
        }
    }
}
