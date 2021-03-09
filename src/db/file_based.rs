use std::fs;
use std::error::Error;

use crate::model::{Event, EventFilter, Comment, CommentFilter};
use super::EventDb;

static EVENTS_JSON: &str = "data/events.json";
static COMMENTS_JSON: &str = "data/comments.json";

pub struct FileBasedEventDb {}

impl EventDb for FileBasedEventDb {

    fn get_events(&self, filter: Option<EventFilter>) -> Result<Vec<Event>, Box<dyn Error>> {
        filter_events(filter)
    }

    fn get_event(&self, event_id: String) -> Result<Event, Box<dyn Error>> {
        let events = read_events()?;
        let matches: Vec<Event> = events
            .into_iter()
            .filter(|e| e.id == Some(event_id.to_string()))
            .collect();
        
        Ok(matches.first().unwrap().clone())
    }

    fn create_event(&self, event: Event) -> Result<Event, Box<dyn Error>> {
        let mut events = read_events()?;
        let mut new_event = event.clone();
        new_event.id = Some(super::create_uuid());
        events.push(new_event.clone());
        write_events(events);
        Ok(new_event)
    }

    fn update_event(&self, event: Event) -> Result<Event, Box<dyn Error>> {
        // TODO: Do not allow updating without event.id, throw error
        let events = read_events()?;
        let updates = events
            .into_iter()
            .map(|e| {
                if e.id == event.id {
                    return event.clone();
                } else {
                    return e;
                }
            })
            .collect();
        write_events(updates);
        Ok(event)
    }

    fn delete_event(&self, event_id: String) -> Result<bool, Box<dyn Error>> {
        // Filter to this event and remove it
        Ok(false)
    }
    
    fn get_comments(&self, filter: Option<CommentFilter>) -> Result<Vec<Comment>, Box<dyn Error>> {
        let comments = read_comments()?;
        if filter.is_none() {
            return Ok(comments);
        }

        let filter = filter.unwrap();
        let event_id = filter.event_id.unwrap_or("".to_string());
        let user_id = filter.user_id.unwrap_or("".to_string());
            
        Ok(comments
            .into_iter()
            .filter(|comment| {
                if event_id != "" && event_id != comment.event_id {
                    return false;
                }
                if user_id != "" && user_id != comment.user_id {
                    return false;
                }
                return true;
            })
            .collect())
    }

    fn get_comment(&self, comment_id: String) -> Result<Comment, Box<dyn Error>> {
        // TODO: Call self.get_comments with id in filter
        let comments = read_comments()?;
        let matches: Vec<Comment> = comments
            .into_iter()
            .filter(|c| c.id == Some(comment_id.to_string()))
            .collect();
        Ok(matches.first().unwrap().clone())
    }

    fn create_comment(&self, comment: Comment) -> Result<Comment, Box<dyn Error>> {
        let mut comments = read_comments()?;
        let mut new_comment = comment.clone();
        new_comment.id = Some(super::create_uuid());
        comments.push(new_comment.clone());
        write_comments(comments);
        Ok(new_comment)
    }

    fn update_comment(&self, comment: Comment) -> Result<Comment, Box<dyn Error>> {
        // Filter to this comment and replace it
        Ok(comment)
    }

    fn delete_comment(&self, event_id: String) -> Result<bool, Box<dyn Error>> {
        // Filter to this comment and remove it
        Ok(false)
    }
}

fn read_events() -> Result<Vec<Event>, Box<dyn Error>> {
    let data = fs::read_to_string(EVENTS_JSON).expect("Error reading from events file.");
    let events: Vec<Event> = serde_json::from_str(&data)?;
    Ok(events)
}

fn write_events(events: Vec<Event>) {
    let data = serde_json::to_string(&events).expect("Failed to serialize events.");
    fs::write(EVENTS_JSON, data).expect("Failed to write events file.");
}

fn filter_events(filter: Option<EventFilter>) -> Result<Vec<Event>, Box<dyn Error>> {
    let events = read_events()?;
    if filter.is_none() {
        return Ok(events);
    }
    let filter = filter.unwrap();
    Ok(events
       .into_iter()
       .filter(|event| {
           if filter.app_name.is_some() && event.app_name != filter.app_name {
               return false;
           }
           if filter.source_id.is_some() && event.source_id != filter.source_id {
               return false;
           }
           if filter.source_name.is_some() && event.source_name != filter.source_name {
               return false;
           }
           if filter.from.is_some() {
               if event.from < filter.from.unwrap() && event.to.is_some() && event.to.unwrap() < filter.from.unwrap() {
                   return false;
               }
           }
           if filter.to.is_some() {
               if event.from > filter.to.unwrap() || (event.to.is_some() && event.to.unwrap() > filter.to.unwrap()) {
                   return false;
               }
           }
           return true;
       })
       .collect())
}

fn read_comments() -> Result<Vec<Comment>, Box<dyn Error>> {
    let data = fs::read_to_string(COMMENTS_JSON).expect("Error reading from comments file.");
    let comments: Vec<Comment> = serde_json::from_str(&data)?;
    Ok(comments)
}

fn write_comments(comments: Vec<Comment>) {
    let data = serde_json::to_string(&comments).expect("Failed to serialize comments.");
    fs::write(COMMENTS_JSON, data).expect("Failed to write comments file.");
}

