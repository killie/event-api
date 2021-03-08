use std::fs;
use uuid::Uuid;
use crate::model::{Event, Comment};
use crate::envelope::{Link, Template, MethodType, Property};

static EVENTS_JSON: &str = "data/events.json";
static COMMENTS_JSON: &str = "data/comments.json";

type EventsResult = Result<Vec<Event>, serde_json::Error>;
type EventsOption = Option<Vec<Event>>;
type CommentsResult = Result<Vec<Comment>, serde_json::Error>;
type CommentsOption = Option<Vec<Comment>>;

fn read_events(include_affordance: bool) -> EventsResult {
    let data = fs::read_to_string(EVENTS_JSON).expect("Error reading from events file.");
    let events: EventsResult = serde_json::from_str(&data);
    if include_affordance {
        match events {
            Ok(events) => Ok(events.into_iter().map(add_affordances).collect()),
            Err(_) => events,
        }
    } else {
        events
    }
}

fn write_events(events: Vec<Event>) {
    let data = serde_json::to_string(&events).expect("Failed to serialize events.");
    fs::write(EVENTS_JSON, data).expect("Failed to write events file.");
}

fn create_uuid() -> String {
    Uuid::new_v4().to_hyphenated().to_string()
}

pub fn get_events() -> EventsOption {
    match read_events(true) {
        Ok(events) => Some(events),
        Err(_) => None
    }
}

fn add_affordances(mut event: Event) -> Event {
    match event.id {
        Some(ref id) => {
            let mut links: Vec<Link> = Vec::new();
            let mut href = "/events/".to_string();
            href.push_str(&id);
            links.push(Link { key: "self".to_string(), href: href.clone() });
            href.push_str("/comments");
            links.push(Link { key: "comments".to_string(), href: href });
            event._links = Some(links);

            let mut templates: Vec<Template> = Vec::new();
            templates.push(Template {
                key: "delete".to_string(),
                title: None,
                method: MethodType::DELETE,
                properties: None,
                target: Some("self".to_string()),
            });
            templates.push(Template {
                key: "update".to_string(),
                title: None,
                method: MethodType::PATCH,
                properties: Some(get_update_properties()),
                target: Some("self".to_string()),
            });
            templates.push(Template {
                key: "comment".to_string(),
                title: None,
                method: MethodType::POST,
                properties: Some(get_comment_properties(&id)),
                target: Some("comments".to_string()),
            });
            event._templates = Some(templates);

            event
        },
        None => event
    }
}

fn get_update_properties() -> Vec<Property> {
    let mut properties: Vec<Property> = Vec::new();
    properties.push(Property {
        name: "from".to_string(), prompt: None, read_only: false, required: true, templated: None, value: None
    });
    properties.push(Property {
        name: "to".to_string(), prompt: None, read_only: false, required: false, templated: None, value: None
    });
    properties.push(Property {
        name: "text".to_string(), prompt: None, read_only: false, required: true, templated: None, value: None
    });
    properties.push(Property {
        name: "appName".to_string(), prompt: None, read_only: false, required: false, templated: None, value: None
    });
    properties.push(Property {
        name: "sourceId".to_string(), prompt: None, read_only: false, required: false, templated: None, value: None
    });
    properties.push(Property {
        name: "sourceName".to_string(), prompt: None, read_only: false, required: false, templated: None, value: None
    });
    properties
}

fn get_comment_properties(id: &str) -> Vec<Property> {
    let mut properties: Vec<Property> = Vec::new();
    properties.push(Property {
        name: "eventId".to_string(), prompt: None, read_only: true, required: true, templated: None, value: Some(id.to_string())
    });
    properties.push(Property {
        name: "userId".to_string(), prompt: None, read_only: false, required: true, templated: None, value: None
    });
    properties.push(Property {
        name: "comment".to_string(), prompt: None, read_only: false, required: true, templated: None, value: None
    });
    properties.push(Property {
        name: "timestamp".to_string(), prompt: None, read_only: false, required: false, templated: None, value: None
    });
    properties
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
    match read_events(true) {
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
    match read_events(true) {
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
    match read_events(false) {
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
