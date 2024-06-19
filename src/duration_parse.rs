use std::{str::FromStr, time::Duration};

#[derive(Debug, Clone)]
pub struct TimeInterval {
    hours: u64,
    minutes: u64,
    seconds: u64,
}

impl TimeInterval {
    pub fn to_duration(&self) -> Duration {
        Duration::new(self.hours * 3600 + self.minutes * 60 + self.seconds, 0)
    }

    fn parse_from_str(interval_str: &str) -> Result<Self, String> {
        let mut hours = 0;
        let mut minutes = 0;
        let mut seconds = 0;

        let mut parts = interval_str.splitn(2, ':');
        let duration_part = parts.next().unwrap_or("");
        let unit_part = parts.next().unwrap_or("").to_lowercase();

        // Handle hours and minutes
        if let Ok(value) = duration_part.parse::<u64>() {
            if unit_part.contains("h") || unit_part.contains("hour") {
                hours = value;
            } else if unit_part.contains("m") || unit_part.contains("minute") {
                minutes = value;
            } else {
                return Err(format!("Invalid unit: {}", unit_part));
            }
        } else {
            return Err(format!("Invalid duration: {}", duration_part));
        }

        // Handle seconds (optional)
        if let Some(seconds_part) = parts.next() {
            if let Ok(value) = seconds_part.parse::<u64>() {
                seconds = value;
                if unit_part.is_empty() && seconds > 59 {
                    return Err(format!("Seconds cannot exceed 59"));
                }
            } else {
                return Err(format!("Invalid seconds: {}", seconds_part));
            }
        }

        Ok(TimeInterval {
            hours,
            minutes,
            seconds,
        })
    }
}

impl FromStr for TimeInterval {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TimeInterval::parse_from_str(s)
    }
}
