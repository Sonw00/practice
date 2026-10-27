use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock{
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (carried_hours, result_minutes) = if minutes >= 0 {
            (minutes / 60, minutes % 60)
        } else {
            (minutes / 60 - 1, minutes % 60 + 60)
        };

        let sum_hours = hours + carried_hours;

        let result_hours = if (sum_hours) >= 0 {
            (sum_hours) % 24
        } else {
            (sum_hours) % 24 + 24
        };

        Self{hours: result_hours, minutes: result_minutes}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let sum_minutes = self.minutes + minutes;

        let (carried_hours, result_minutes) = if sum_minutes >= 0 {
            (sum_minutes / 60, sum_minutes % 60)
        } else {
            (sum_minutes / 60 - 1, sum_minutes % 60 + 60)
        };

        let sum_hours = self.hours + carried_hours;

        let result_hours = if (sum_hours) >= 0 {
            (sum_hours) % 24
        } else {
            (sum_hours) % 24 + 24
        };

        Self{hours: result_hours, minutes: result_minutes}
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hours: String = if self.hours >= 10 {
            self.hours.to_string()
        } else {
            "0".to_owned() + &self.hours.to_string()
        };
        
        let minutes: String = if self.minutes >= 10 {
            self.minutes.to_string()
        } else {
            "0".to_owned() + &self.minutes.to_string()
        };

        write!(f, "{}:{}", hours, minutes)
    }
}
