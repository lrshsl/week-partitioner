pub(crate) struct Time {
    minutes: i32,
}

impl std::ops::Add<Time> for Time {
    type Output = Self;

    fn add(self, rhs: Time) -> Self {
        Self {
            minutes: self.minutes + rhs.minutes,
        }
    }
}

impl std::ops::Sub<Time> for Time {
    type Output = Self;

    fn sub(self, rhs: Time) -> Self::Output {
        Self {
            minutes: self.minutes - rhs.minutes,
        }
    }
}

impl Time {
    pub const fn from_hours(hours: i32) -> Self {
        Self {
            minutes: hours * 60,
        }
    }

    pub const fn from_minutes(minutes: i32) -> Self {
        Self { minutes }
    }

    pub const fn hours_precise(&self) -> f32 {
        (self.minutes as f32) / 60.0
    }

    pub const fn hours(&self) -> i32 {
        self.minutes / 60
    }

    pub const fn minutes(&self) -> i32 {
        self.minutes
    }
}
pub(crate) const fn hours(h: i32) -> Time {
    Time::from_hours(h)
}
pub(crate) const fn minutes(m: i32) -> Time {
    Time::from_minutes(m)
}
