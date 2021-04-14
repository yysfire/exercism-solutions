use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hour: i32,
    minute: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (hours_extra, minute) = match minutes >= 0 {
            true => (minutes / 60, minutes % 60),
            false => (minutes / 60 - 1, minutes % 60 + 60),
        };
        let mut hour = (hours + hours_extra) % 24;
        if hour < 0 {
            hour += 24;
        }
        Clock {hour, minute}
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        Clock::new(self.hour, self.minute + minutes)
    }
}


//impl ToString for Clock {

//    fn to_string(&self) -> String {
//        format!("{:02}:{:02}", self.hour, self.minute)
//    }
//}


impl fmt::Display for Clock {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hour, self.minute)
    }
}


impl From<Clock> for String {
    fn from(clock: Clock) -> Self {
        clock.to_string()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_string_from_clock() {
        assert_eq!(String::from(Clock::new(8, 0)), "08:00".to_owned());
    }
}
