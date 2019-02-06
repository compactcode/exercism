use std::fmt;

#[derive(PartialEq, Debug)]
pub struct Clock {
    total_minutes: i32
}

fn modulo(n: i32, modulus: i32) -> i32 {
    ((n % modulus) + modulus) % modulus
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            total_minutes: modulo(hours * 60 + minutes, 60 * 24)
        }
    }

    pub fn hours(&self) -> i32 {
        self.total_minutes / 60
    }

    pub fn minutes(&self) -> i32 {
        self.total_minutes % 60
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        Clock::new(self.hours(), self.minutes() + minutes)
    }

}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours(), self.minutes())
    }
}
