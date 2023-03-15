use crate::models::{Event, Provider};
use google_calendar3::{CalendarHub};
use std::error::Error;
use yup_oauth2::InstalledFlowAuthenticator;

pub struct GoogleCalendar {
    calendar_hub: CalendarHub,
    calendar_id: String,
}

impl GoogleCalendar {
    pub async fn new(credentials: &str, calendar_id: &str) -> Result<Self, Box<dyn Error>> {
        let secrets = yup_oauth2::read_application_secret(credentials.as_bytes())?;
        let auth = InstalledFlowAuthenticator::builder(secrets, yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect)
            .build()
            .await?;

        let client = hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots());
        let calendar_hub = CalendarHub::new(client, auth);

        Ok(Self {
            calendar_hub,
            calendar_id: calendar_id.to_string(),
        })
    }
}

impl Provider for GoogleCalendar {
    fn get_events(&self) -> Result<Vec<Event>, Box<dyn Error>> {
        self.list_events()
    }

    fn get_event(&self, event_id: String) -> Result<Event, Box<dyn Error>> {
        // Implement the method to fetch a single event from Google Calendar
    }

    fn add_event(&self, event: Event) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn delete_event(&self, event_id: String) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
