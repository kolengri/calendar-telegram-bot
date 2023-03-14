use crate::models::event::Event;
use crate::provider::Provider;
use google_calendar3::{Event as GoogleEvent, GoogleCalendarHub};
use std::error::Error;

pub struct GoogleCalendar {
    hub: GoogleCalendarHub,
}

impl GoogleCalendar {
    pub fn new(client_secret_path: &str, token_path: &str) -> Result<GoogleCalendar, Box<dyn Error>> {
        // Create a new Google Calendar API client using the client secret and token paths
        let hub = GoogleCalendarHub::new(
            hyper::Client::builder().build(hyper_tls::HttpsConnector::new()?),
            yup_oauth2::read_client_secret(client_secret_path)?,
            yup_oauth2::read_token_from_file(token_path).ok(),
        );

        Ok(GoogleCalendar { hub })
    }
}

impl Provider for GoogleCalendar {
    fn get_events(&self) -> Result<Vec<Event>, Box<dyn Error>> {
        // Call the Google Calendar API to get the list of events
        // ...

        Ok(Vec::new())
    }

    fn get_event(&self, event_id: String) -> Result<Event, Box<dyn Error>> {
        // Call the Google Calendar API to get the event with the specified ID
        // ...

        Ok(Event::default())
    }

    fn add_event(&self, event: Event) -> Result<(), Box<dyn Error>> {
        // Convert the `Event` object to a `GoogleEvent` object
        let google_event = GoogleEvent::default();

        // Call the Google Calendar API to add the event
        // ...

        Ok(())
    }

    fn delete_event(&self, event_id: String) -> Result<(), Box<dyn Error>> {
        // Call the Google Calendar API to delete the event with the specified ID
        // ...

        Ok(())
    }
}
