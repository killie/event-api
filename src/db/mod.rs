use std::error::Error;
use uuid::Uuid;

use crate::model::{Event, EventFilter, Comment, CommentFilter};

pub mod file_based;

pub trait EventDb {
    fn get_events(&self, filter: Option<EventFilter>) -> Result<Vec<Event>, Box<dyn Error>>;
    fn get_event(&self, event_id: String) -> Result<Event, Box<dyn Error>>;
    fn create_event(&self, event: Event) -> Result<Event, Box<dyn Error>>;
    fn update_event(&self, event: Event) -> Result<Event, Box<dyn Error>>;
    fn delete_event(&self, event_id: String) -> Result<bool, Box<dyn Error>>;
    fn get_comments(&self, filter: Option<CommentFilter>) -> Result<Vec<Comment>, Box<dyn Error>>;
    fn get_comment(&self, comment_id: String) -> Result<Comment, Box<dyn Error>>;
    fn create_comment(&self, comment: Comment) -> Result<Comment, Box<dyn Error>>;
    fn update_comment(&self, comment: Comment) -> Result<Comment, Box<dyn Error>>;
    fn delete_comment(&self, comment_id: String) -> Result<bool, Box<dyn Error>>;
}

fn create_uuid() -> String {
    Uuid::new_v4().to_hyphenated().to_string()
}
