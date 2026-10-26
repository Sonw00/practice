use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock{
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let carried_hours = minutes / 60;
        Self{hours: hours + carried_hours, minutes: minutes % 60}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let carried_hours = (self.minutes + minutes) / 60;
        let remained_minutes = (self.minutes + minutes) % 60;

        Self{hours: self.hours + carried_hours, minutes: remained_minutes}
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.hours, self.minutes)
    }
}
