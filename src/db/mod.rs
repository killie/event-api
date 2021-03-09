use std::error::Error;
use uuid::Uuid;

use crate::model::{Event, Comment};
use crate::envelope::{Link, Template, MethodType, Property};

pub mod file_based;

pub struct EventFilter {
    pub from: Option<i64>,
    pub to: Option<i64>,
    pub app_name: Option<String>,
    pub source_id: Option<String>,
    pub source_name: Option<String>,
}

pub struct CommentFilter {
    pub event_id: Option<String>,
    pub user_id: Option<String>,
}

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
