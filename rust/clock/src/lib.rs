use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: u32,
    minutes: u32,
}

impl Clock {
    pub fn new(h: i32, m: i32) -> Self {
        let clock = Clock {
            hours: 0 as u32,
            minutes: 0 as u32,
        };

        clock.add_hours_and_minutes(h, m)
    }

    pub fn add_minutes(&self, m: i32) -> Self {
        self.add_hours_and_minutes(0, m)
    }

    fn add_hours_and_minutes(&self, h: i32, m: i32) -> Self {
        let mut h_mut = self.hours as i32 + h;
        let mut m_mut = self.minutes as i32 + m;

        h_mut += m_mut/60;
        if m_mut >= 0 {
            m_mut = m_mut % 60;
        } else {
            m_mut = 60 - m_mut.abs() % 60;
            h_mut -= 1;
        }

        if h_mut >= 0 {
            h_mut = h_mut % 24;
        } else {
            h_mut = (24 - h_mut.abs() % 24)%24;
        }

        
        Clock {
            hours: h_mut as u32,
            minutes: m_mut as u32,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Clock) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}
