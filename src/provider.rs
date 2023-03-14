use crate::models::event::Event;
use std::error::Error;

pub trait Provider {
    fn get_events(&self) -> Result<Vec<Event>, Box<dyn Error>>;
    fn get_event(&self, event_id: String) -> Result<Event, Box<dyn Error>>;
    fn add_event(&self, event: Event) -> Result<(), Box<dyn Error>>;
    fn delete_event(&self, event_id: String) -> Result<(), Box<dyn Error>>;
}
