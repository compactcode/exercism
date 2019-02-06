use std::fmt;

#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

fn modulo(n: i32, modulus: i32) -> i32 {
    ((n % modulus) + modulus) % modulus
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mins = modulo(hours * 60 + minutes, 1440);

        Clock {
            hours: (mins / 60) % 24,
            minutes: mins % 60
        }
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }

}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
