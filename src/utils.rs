use chrono::{DateTime, Local, Datelike, Timelike, TimeZone};
use serde::{Serialize, Deserialize};

use pelican_ui::drawable::Drawable;
use pelican_ui::Context;

pub trait AppPage: Drawable + std::fmt::Debug + 'static {
    fn navigate(self: Box<Self>, ctx: &mut Context, index: usize) -> Result<Box<dyn AppPage>, Box<dyn AppPage>>;
    fn has_nav(&self) -> bool;
}

// pub use pelican_macro::AppPage as derive_AppPage;

// pub mod macros {
//     pub use pelican_macro::AppPage;
// }

// #[derive(Clone, Copy, Deserialize, Serialize, Debug)]
// pub struct InternetConnection(pub bool);

/// `Timestamp` contains the date time in an easy-to-read format.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Timestamp(String, String);

impl Timestamp {
    /// Create a `Timestamp` from a local [`DateTime<Local>`].
    ///
    /// Formats as `M/D/YY` for the date and `H:MM AM/PM` for the time.
    pub fn new(dt: DateTime<Local>) -> Self {
        Timestamp(
            dt.format("%-m/%-d/%y").to_string(), 
            dt.format("%-I:%M %p").to_string()
        )
    }

    /// Create a `Timestamp` with date and time set as pending (`"-"`).
    pub fn pending() -> Self {
        Timestamp("-".to_string(), "-".to_string())
    }

    /// Tries to convert the `Timestamp` into a `DateTime<Local>`.
    ///
    /// Parses the stored date and time strings using the format `M/D/YY H:MM AM/PM`.
    pub fn to_datetime(&self) -> Option<DateTime<Local>> {
        let combined = format!("{} {}", self.date(), self.time());
        let format = "%m/%d/%y %I:%M %p";
        let naive = chrono::NaiveDateTime::parse_from_str(&combined, format).expect("Could not parse time");
        Local.from_local_datetime(&naive).single()
    }

    /// Returns a human-readable, "direct" representation of the timestamp.
    ///
    /// Formats the timestamp based on how recent it is:
    /// - **Today**: `"H:MM am/pm"`
    /// - **Yesterday**: `"yesterday, H:MM am/pm"`
    /// - **Same week**: day of the week (e.g., `"Monday"`)
    /// - **Same year**: `"Month D"` (e.g., `"August 16"`)
    /// - **Otherwise**: `"MM/DD/YY"`
    ///
    /// Returns `None` if the timestamp cannot be converted to a local datetime.
    pub fn direct(&self) -> Option<String> {
        let dt = self.to_datetime()?;
        let today = Local::now().date_naive();
        let date = dt.date_naive();
        let hour = dt.hour();
        let minute = dt.minute();
        let (hour12, am_pm) = match hour == 0 {
            true => (12, "am"),
            false if hour < 12 => (hour, "am"),
            false if hour == 12 => (12, "pm"),
            false => (hour - 12, "pm")
        };

        let the_time = format!("{hour12}:{minute:02} {am_pm}");

        match date == today {
            true => the_time.into(),
            false if date == today.pred_opt().unwrap_or(today) => format!("yesterday, {the_time}").into(),
            false if date.iso_week() == today.iso_week() => format!("{}", dt.format("%A")).into(),
            false if date.year() == today.year() => format!("{}", dt.format("%B %-d")).into(),
            false => format!("{}", dt.format("%m/%d/%y")).into()
        }
    }

    /// Returns a “friendly” human-readable representation of the timestamp.
    ///
    /// Formats the timestamp based on how recent it is:
    /// - **Today:** `"H:MM AM/PM"`
    /// - **Yesterday:** `"Yesterday"` (time omitted)
    /// - **Same week:** day of the week (e.g., `"Monday"`)
    /// - **Same year:** `"Month D"` (e.g., `"August 16"`)
    /// - **Other years:** `"MM/DD/YY"`
    ///
    /// Returns `None` if the timestamp cannot be converted to a local datetime.
    pub fn friendly(&self) -> Option<String> {
        let dt = self.to_datetime()?;
        let today = Local::now().date_naive();
        let date = dt.date_naive();

        match date == today {
            true => {
                let hour = dt.hour();
                let minute = dt.minute();
                let (hour12, am_pm) = match hour == 0 {
                    true => (12, "AM"),
                    false if hour < 12 => (hour, "AM"),
                    false if hour == 12 => (12, "PM"),
                    false => (hour - 12, "PM")
                };
                format!("{hour12}:{minute:02} {am_pm}").into()
            },
            false if date == today.pred_opt().unwrap_or(today) => "Yesterday".to_string().into(),
            false if date.iso_week() == today.iso_week() => format!("{}", dt.format("%A")).into(),
            false if date.year() == today.year() => format!("{}", dt.format("%B %-d")).into(),
            false => format!("{}", dt.format("%m/%d/%y")).into()
        }
    }

    /// Returns the date.
    pub fn date(&self) -> String {self.0.clone()}
    /// Returns the time.
    pub fn time(&self) -> String {self.1.clone()}
}

pub type Callback = Box<dyn FnMut(&mut Context)>;

/// Represents a unique identifier for an element in the user interface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ElementID(uuid::Uuid);

impl ElementID {
    /// A new `ElementID` with a random UUID.
    pub fn new() -> Self {
        ElementID(uuid::Uuid::new_v4())
    }

    /// Returns the underlying UUID of the `ElementID`.
    pub fn as_uuid(&self) -> uuid::Uuid {
        self.0
    }
}

impl Default for ElementID {
    fn default() -> Self {
        Self::new()
    }
}

