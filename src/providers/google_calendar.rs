use crate::models::{Event, Provider};
use std::error::Error;
use google_calendar3::api::Events as GoogleEvents;
use google_calendar3::CalendarHub;
use yup_oauth2::{ServiceAccountKey, ServiceAccountAuthenticator};
use hyper::client::connect::Connect;
use chrono::{DateTime, Utc};

pub struct GoogleCalendarProvider<C>
where
    C: Connect + Clone + Send + Sync + 'static,
{
    calendar_hub: CalendarHub<C>,
    calendar_id: String,
}

impl<C> GoogleCalendarProvider<C>
where
    C: Connect + Clone + Send + Sync + 'static,
{
    pub async fn new(credentials_path: &str, calendar_id: &str) -> Result<Self, Box<dyn Error>> {
        let secret = yup_oauth2::read_service_account_key(credentials_path).await?;
        let auth = ServiceAccountAuthenticator::builder(secret)
            .build()
            .await?;
        let auth = auth.with_scopes(vec![
            "https://www.googleapis.com/auth/calendar",
            "https://www.googleapis.com/auth/calendar.events",
        ]);
        let connector = hyper_rustls::HttpsConnector::with_native_roots();
        let client = hyper::Client::builder().build::<_, hyper::Body>(connector.clone());
        let hub = CalendarHub::new(client, auth);
        Ok(GoogleCalendarProvider {
            calendar_hub: hub,
            calendar_id: calendar_id.to_owned(),
        })
    }

    fn convert_google_event_to_event(google_event: google_calendar3::api::Event) -> Option<Event> {
        let id = google_event.id?;
        let title = google_event.summary?;
        let start_time = google_event.start?.date_time?;
        let end_time = google_event.end?.date_time?;
    
        Some(Event {
            id,
            title,
            start_time,
            end_time,
        })
    }
    
    async fn fetch_google_events(
        calendar_hub: &CalendarHub<C>,
        calendar_id: &str,
    ) -> Result<GoogleEvents, Box<dyn Error>> {
        let calendar_hub = calendar_hub.events();
        let events = calendar_hub
            .list(calendar_id)
            .max_results(100)
            .single_events(true)
            .order_by("startTime")
            .time_min(DateTime::<Utc>::from_utc(
                Utc::now(),
                Utc,
            ))
            .doit()
            .await?;
    
        Ok(events)
    }
}
