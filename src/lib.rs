use std::cmp::min;
use std::fmt::{Display, Formatter};

mod tests;

#[derive(PartialEq, Eq, Debug)]
pub struct Clock {minutes: i32}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / Self::HOUR, self.minutes % Self::HOUR)
    }
}

impl Clock {
    const HOUR: i32 = 60;
    const DAY: i32 = Self::HOUR * 24;
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {minutes: (Self::DAY + ((minutes + hours * Self::HOUR) % Self::DAY)) % Self::DAY}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {minutes: (Self::DAY + ((minutes + self.minutes) % Self::DAY)) % Self::DAY}
    }
}
