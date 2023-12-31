use core::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let clock = Clock {
            hours,
            minutes
        };
        let adjusted_clock = clock.adjust();
        adjusted_clock.validate();
        return adjusted_clock
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let summed_minutes: i32 = self.minutes + minutes;
        Clock::new(self.hours, summed_minutes)
    }

    fn adjust(&self) -> Clock {
        let delta_hours = self.minutes / 60;
        let remainder_minutes = self.minutes % 60;
        let negative_minutes_hour_adjust;
        let minutes;
        let hours;
        if remainder_minutes < 0 {
            minutes = 60 + remainder_minutes;
            negative_minutes_hour_adjust = -1
        } else {
            minutes = remainder_minutes;
            negative_minutes_hour_adjust = 0
        }
        let remainder_hours = (self.hours + delta_hours) % 24;
        if remainder_hours + negative_minutes_hour_adjust < 0 {
            hours = 24 + remainder_hours + negative_minutes_hour_adjust
        } else {
            hours = remainder_hours + negative_minutes_hour_adjust
        }
        let clock = Clock {
            hours,
            minutes
        };
        return clock
    }

    fn validate(&self) {
        if self.hours < 0 || self.hours > 23 {
            panic!("Invalid hours value {}", self.hours)
        }
        if self.minutes < 0 || self.minutes > 59 {
            panic!("Invalid minutes value {}", self.minutes)
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}
