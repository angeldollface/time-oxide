/*
TIME OXIDE by Alexander Abraham, a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

// Importing Chrono's
// APIs and types to get time.
use chrono::DateTime;
use chrono::Datelike;
use chrono::Timelike;
use chrono::offset::Utc;

// A "struct" to hold data
// about the time and date.
#[derive(Debug, PartialEq)]
pub struct Now{
    pub date: String,
    pub time: String
}

impl Now {

    // Called when a new instance
    // of the "struct" is created.
    pub fn new() -> Self {
        return Now{
            date: Now::date(),
            time: Now::time()
        };
    }

    // String representation of
    // our "struct".
    pub fn to_string(&self) -> String {
        return format!(
            "{}\n{}",
            self.date,
            self.time
        );
    }

    // Gets the current date.
    pub fn date() -> String {
        let mut result: String = String::from("");
        let now: DateTime<Utc> = chrono::Utc::now();
        let current_year: String = now.year().to_string();
        let current_month: String = now.month().to_string();
        let current_day: String = now.day().to_string();
        result = format!(
            "{}/{}/{}",
            current_year,
            current_month,
            current_day
        );
        return result;
    }

    // Gets seconds as a string.
    pub fn seconds() -> String {
        let mut result: String = String::from("");
        let now: DateTime<Utc> = chrono::Utc::now();
        let current_seconds: String = now.second().to_string();
        result = format!(
            "{}",
            current_seconds
        );
        return result;
    }

    // Gets minutes as a string.
    pub fn minutes() -> String {
        let mut result: String = String::from("");
        let now: DateTime<Utc> = chrono::Utc::now();
        let current_minutes: String = now.minute().to_string();
        result = format!(
            "{}",
            current_minutes
        );
        return result;
    }

    // Gets hours as a string.
    pub fn hours() -> String {
        let mut result: String = String::from("");
        let now: DateTime<Utc> = chrono::Utc::now();
        let current_hour: String = now.hour().to_string();
        result = format!(
            "{}",
            current_hour,
        );
        return result;
    }

    // Combines all three of the above
    // methods to get time as string.
    pub fn time() -> String {
        return format!{
            "{}:{}:{}",
            Now::hours(),
            Now::minutes(),
            Now::seconds()
        }
    }
}