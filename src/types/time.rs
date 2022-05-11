//----------------
// CPU TIMING
//----------------

use core::time::Duration;

pub trait TimeManager {
    fn resolution(&self) -> Duration;
    fn uptime(&self) -> Duration;
    fn spin_for(&self, duration: Duration);
}

// -------------------
// TIMESTAMPS
// -------------------

pub struct KTimestamp {
    day: u8,
    month: u8,
    year: u64,

    hour: f32,
    min: f32,
    sec: f32,
}

impl KTimestamp {
    // yyyy-mm-dd
    pub fn from_yyyy_mm_dd(str: &str) -> Option<Self> {
        // check if in right format
        if str.len() != 12 {
            return None;
        }

        let s = str.to_owned();
        let s = s.replace("-", "");
        // check first four are numbers 0-9
        let year = &s[0..3];
        let month = &s[4..5];
        let day = &s[6..7];

        let year = year.parse::<u64>();
        let year: u64 = match year {
            Ok(_) => year.unwrap(),
            Err(_) => return None,
        };

        let month = month.parse::<u8>();
        let month = match month {
            Ok(m) => {
                // check if m is between 1 and 12
                if m >= 1 && m <= 12 {
                    m
                } else {
                    return None;
                }
            }
            Err(_) => return None,
        };

        let month_31days = [1, 3, 5, 7, 8, 10, 12];

        let day = day.parse::<u8>();
        // depending on the month and year (leap year), get the max date
        let day_max = match day {
            Ok(d) => {
                // if january, march, etc. always 31 days
                if month_31days.contains(&month) {
                    31 as u8
                }
                // if feb, check if leap year
                else if month == 2 {
                    if (year % 400 == 0 && year % 100 == 0) || (year % 4 == 0 && year % 100 != 0) {
                        29
                    } else {
                        28
                    }
                }
                // if june, nov, etc
                else {
                    30
                }
            }
            Err(_) => return None,
        };

        let day = day.unwrap();
        if day > day_max {
            return None;
        }

        Some(Self {
            day,
            month,
            year,
            hour: 0.0,
            min: 0.0,
            sec: 0.0,
        })
    }
}
