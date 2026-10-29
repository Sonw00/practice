use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock{
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut carried_hours = minutes / 60;
        let mut remained_minutes = minutes % 60;

        if (remained_minutes) < 0 {
            carried_hours -= 1;
            remained_minutes += 60;
        }

        let sum_hours = (hours + carried_hours) % 24;

        let result_hours = if (sum_hours) >= 0 {
            sum_hours
        } else {
            sum_hours + 24
        };

        Self{hours: result_hours, minutes: remained_minutes}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let sum_minutes = self.minutes + minutes;

        let mut carried_hours = sum_minutes / 60;
        let mut remained_minutes = sum_minutes % 60;

        if remained_minutes < 0 {
            carried_hours -= 1;
            remained_minutes += 60;
        }

        let sum_hours = (self.hours + carried_hours) % 24;

        let result_hours = if (sum_hours) >= 0 {
            sum_hours
        } else {
            sum_hours + 24
        };

        Self{hours: result_hours, minutes: remained_minutes}
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
