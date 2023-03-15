use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub summary: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub start_time: DateTime<Local>,
    pub end_time: DateTime<Local>,
    pub utc_offset: u16,
}

impl Event {
    pub fn new(
        summary: String,
        start_time: DateTime<Local>,
        end_time: DateTime<Local>,
        description: Option<String>,
        location: Option<String>,
        utc_offset: u16
    ) -> Event {
        Event {
            summary,
            description,
            location,
            start_time,
            end_time,
            utc_offset,
        }
    }

    pub fn default() -> Event {
        Event {
            summary: String::new(),
            description: None,
            location: None,
            start_time: Local::now(),
            end_time: Local::now(),
            utc_offset: 0,
        }
    }
}
