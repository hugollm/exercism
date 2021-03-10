#[derive(Debug)]
#[derive(PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {

    pub fn new(h: i32, m: i32) -> Self {
        let rollover = m / 60;
        let mut hours = (h + rollover) % 24;
        let mut minutes = m % 60;
        if minutes < 0 {
            hours -= 1;
            minutes += 60;
        }
        if hours < 0 {
            hours += 24;
        }
        Clock{ hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }
}

impl ToString for Clock {

    fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }
}
