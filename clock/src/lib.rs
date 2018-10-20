use std::fmt;

#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            hours: get_hours(hours, minutes),
            minutes: get_minutes(minutes),
        }
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        Clock {
            hours: get_hours(self.hours, self.minutes + minutes),
            minutes: get_minutes(self.minutes + minutes),
        }
    }
}

fn get_hours(h: i32, m: i32) -> i32 {
    ((24 + h % 24) + (24 + ((m - get_minutes(m)) / 60) % 24)) % 24
}

fn get_minutes(m: i32) -> i32 {
    (60 + m % 60) % 60
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
