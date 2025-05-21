use chrono::{DateTime, Local, Datelike, Timelike, TimeZone};
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Timestamp(String, String); // date, time (move to pelican)

impl Timestamp {
    pub fn new(dt: DateTime<Local>) -> Self {
        Timestamp(
            dt.format("%-m/%-d/%y").to_string(), 
            dt.format("%-I:%M %p").to_string()
        )
    }

    pub fn pending() -> Self {
        Timestamp("-".to_string(), "-".to_string())
    }

    pub fn to_datetime(&self) -> DateTime<Local> {
        let combined = format!("{} {}", self.date(), self.time());
        let format = "%m/%d/%y %I:%M %p";
        let naive = chrono::NaiveDateTime::parse_from_str(&combined, format).expect("Could not parse time");
        Local.from_local_datetime(&naive).unwrap()
    }

    pub fn friendly(&self) -> &'static str {
        let dt = self.to_datetime();
        let today = Local::now().date_naive();
        let date = dt.date_naive();

        let result = match date == today {
            true => {
                let hour = dt.hour();
                let minute = dt.minute();
                let (hour12, am_pm) = match hour == 0 {
                    true => (12, "AM"),
                    false if hour < 12 => (hour, "AM"),
                    false if hour == 12 => (12, "PM"),
                    false => (hour - 12, "PM")
                };
                format!("{:02}:{:02} {}", hour12, minute, am_pm)
            },
            false if date == today.pred_opt().unwrap_or(today) => "Yesterday".to_string(),
            false if date.iso_week() == today.iso_week() => format!("{}", dt.format("%A")),
            false if date.year() == today.year() => format!("{}", dt.format("%B %-d")),
            false => format!("{}", dt.format("%m/%d/%y"))
        };

        static_from(result)
    }

    pub fn date(&self) -> &'static str {static_from(self.0.clone())}
    pub fn time(&self) -> &'static str {static_from(self.1.clone())}
}

pub fn static_from(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}