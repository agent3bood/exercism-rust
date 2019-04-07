pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut h = hours;
        let mut m = minutes;
        while h >= 24 || m >= 60 {
            while h >= 24 {
                h -= 24;
            }
            while m >= 60 {
                h += 1;
                m -= 60;
            }
        }
        while h < 0 || m < 0 {
            while h < 0 {
                h = 24 + h;
            }
            while m < 0 {
                h -= 1;
                m += 60;
            }
        }
        Clock {
            hours: h,
            minutes: m,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut h = self.hours;
        let mut m = self.minutes;
        m += minutes;
        while m >= 60 {
            h += 1;
            m -= 60;
            if h == 24 {
                h = 0;
            }
        }
        while m <= -1 {
            h -= 1;
            m += 60;
            if h == -1 {
                h = 23;
            }
        }

        Clock {
            hours: h,
            minutes: m,
        }
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let h;
        let m;
        if self.hours < 10 {
            h = format!("0{}", self.hours);
        } else {
            h = format!("{}", self.hours);
        }
        if self.minutes < 10 {
            m = format!("0{}", self.minutes);
        } else {
            m = format!("{}", self.minutes);
        }
        write!(f, "{}:{}", h, m)
    }
}

impl std::fmt::Debug for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Clock {{ hours: {}, minutes: {} }}",
            self.hours, self.minutes
        )
    }
}

impl std::cmp::PartialEq for Clock {
    fn eq(&self, other: &Clock) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}
