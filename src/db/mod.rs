use std::fs;
use uuid::Uuid;
use crate::model::Event;
use crate::model::Comment;

static EVENTS_JSON: &str = "data/events.json";
static COMMENTS_JSON: &str = "data/comments.json";

type EventsResult = Result<Vec<Event>, serde_json::Error>;
type EventsOption = Option<Vec<Event>>;
type CommentsResult = Result<Vec<Comment>, serde_json::Error>;
type CommentsOption = Option<Vec<Comment>>;

fn read_events() -> EventsResult {
    let data = fs::read_to_string(EVENTS_JSON).expect("Error reading from events file.");
    let events: EventsResult = serde_json::from_str(&data);
    events
}

fn write_events(events: Vec<Event>) {
    let data = serde_json::to_string(&events).expect("Failed to serialize events.");
    fs::write(EVENTS_JSON, data).expect("Failed to write events file.");
}

fn create_uuid() -> String {
    Uuid::new_v4().to_hyphenated().to_string()
}

pub fn get_events() -> EventsOption {
    match read_events() {
        Ok(events) => Some(events),
        Err(_) => None
    }
}

/*
fn filter_events(events: Vec<Event>, f: impl Fn(Event) -> bool) -> EventsOption {
    Some(events
         .into_iter()
         .filter(|e| -> f(e))
         .collect())
}
*/

pub fn query_events(from: Option<i64>, to: Option<i64>, app_name: Option<String>) -> EventsOption {
    match read_events() {
        Ok(events) => {
            Some(events
                 .into_iter()
                 .filter(|e| {

                     if from.is_none() && to.is_none() && app_name.is_none() {
                         return true;
                     }
                     if app_name.is_some() && e.app_name != app_name {
                         return false;
                     }
                     if from.is_some() {
                         if e.from < from.unwrap() && e.to.is_some() && e.to.unwrap() < from.unwrap() {
                             return false;
                         }
                     }
                     if to.is_some() {
                         if e.from > to.unwrap() || (e.to.is_some() && e.to.unwrap() > to.unwrap()) {
                             return false;
                         }
                     }
                     return true;

                 })
                 .collect())

        },
        Err(_) => None,
    }
}
        
pub fn get_event(id: &String) -> EventsOption {
    match read_events() {
        Ok(events) => {
            //let query = |e: Event| e.id == Some(id.to_string());
            //filter_events(events, &query)

            Some(events
                 .into_iter()
                 .filter(|e| e.id == Some(id.to_string()))
                 .collect())

        },
        Err(_) => None,
    }
}

pub fn create_event(event: Event) -> Option<Event> {
    match read_events() {
        Ok(mut events) => {
            let mut new_event = event.clone();
            new_event.id = Some(create_uuid());
            events.push(new_event.clone());
            write_events(events);
            Some(new_event)
        },
        Err(_) => None,
    }
}

fn read_comments() -> CommentsResult {
    let data = fs::read_to_string(COMMENTS_JSON).expect("Error reading from comments file.");
    let comments: CommentsResult = serde_json::from_str(&data);
    comments
}

fn write_comments(comments: Vec<Comment>) {
    let data = serde_json::to_string(&comments).expect("Failed to serialize comments.");
    fs::write(COMMENTS_JSON, data).expect("Failed to write comments file.");
}

pub fn get_comments(event_id: &String) -> CommentsOption {
    match read_comments() {
        Ok(comments) => {
            Some(comments
                 .iter()
                 .filter(|c| c.event_id == event_id.to_string())
                 .cloned()
                 .collect())
        },
        Err(_) => None,
    }
}

pub fn get_comment(id: &String) -> CommentsOption {
    match read_comments() {
        Ok(comments) => {
            Some(comments
                 .iter()
                 .filter(|c| c.id == Some(id.to_string()))
                 .cloned()
                 .collect())
        },
        Err(_) => None,
    }
}

pub fn create_comment(comment: Comment) -> Option<Comment> {
    match read_comments() {
        Ok(mut comments) => {
            let mut new_comment = comment.clone();
            new_comment.id = Some(create_uuid());
            comments.push(new_comment.clone());
            write_comments(comments);
            Some(new_comment)
        },
        Err(_) => None,
    }
}
