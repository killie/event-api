use serde::{Deserialize, Serialize};
use rocket_contrib::json;

use crate::envelope::{Payload, Link, Template, MethodType, Property, create_property};

#[derive(Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Option<String>,
    pub from: i64,
    pub to: Option<i64>,
    pub text: String,
    #[serde(rename = "appName")]
    pub app_name: Option<String>,
    #[serde(rename = "sourceId")]
    pub source_id: Option<String>,
    #[serde(rename = "sourceName")]
    pub source_name: Option<String>,
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    pub _links: Option<Vec<Link>>,
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
    pub _templates: Option<Vec<Template>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: Option<String>,
    #[serde(rename = "eventId")]
    pub event_id: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    pub comment: String,
    pub timestamp: i64,
    #[serde(skip_deserializing)]
    pub _links: Option<Vec<Link>>,
    #[serde(skip_deserializing)]
    pub _templates: Option<Vec<Template>>,
}

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

pub fn get_events_payload(events: Vec<Event>) -> Payload {
    Payload {
        data: json!(events.into_iter().map(extend_event).collect::<Vec<Event>>()),
        links: Some(event_links()),
        templates: Some(event_templates()),
    }
}

pub fn get_event_payload(event: Event) -> Payload {
    let copy = extend_event(event.clone());
    Payload {
        data: json!(event),
        links: copy._links,
        templates: copy._templates,
    }
}

fn extend_event(mut event: Event) -> Event {
    match event.id {
        Some(ref id) => {
            let mut links: Vec<Link> = Vec::new();
            links.push(Link { key: "self".to_string(), href: format!("/events/{}", &id) });
            links.push(Link { key: "comments".to_string(), href: format!("/events/{}/comments", &id) });
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

fn event_links() -> Vec<Link> {
    let mut links: Vec<Link> = Vec::new();
    links.push(Link { key: "self".to_string(), href: "/events".to_string() });
    links
}

fn event_templates() -> Vec<Template> {
    let mut templates: Vec<Template> = Vec::new();
    templates.push(Template {
        key: "default".to_string(),
        title: Some("Create event".to_string()),
        method: MethodType::POST,
        properties: Some(get_create_properties()),
        target: Some("self".to_string())
    });
    templates
}

fn get_create_properties() -> Vec<Property> {
    let mut properties: Vec<Property> = Vec::new();
    properties.push(create_property("from", false, true));
    properties.push(create_property("to", false, false));
    properties.push(create_property("text", false, true));
    properties.push(create_property("appName", false, false));
    properties.push(create_property("sourceId", false, false));
    properties.push(create_property("sourceName", false, false));
    properties
}
    
fn get_update_properties() -> Vec<Property> {
    let mut properties: Vec<Property> = Vec::new();
    properties.push(create_property("from", false, true));
    properties.push(create_property("to", false, false));
    /*
    properties.push(Property {
        name: "from".to_string(), prompt: None, read_only: false, required: true, templated: None, value: None
    });
    properties.push(Property {
        name: "to".to_string(), prompt: None, read_only: false, required: false, templated: None, value: None
    });
     */
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

pub fn get_comments_payload(event_id: String, comments: Vec<Comment>) -> Payload {
    Payload {
        data: json!(comments.into_iter().map(extend_comment).collect::<Vec<Comment>>()),
        links: Some(comment_links(&event_id)),
        templates: Some(comment_templates()),
    }
}

pub fn get_comment_payload(comment: Comment) -> Payload {
    let copy = comment.clone();
    Payload {
        data: json!(comment),
        links: copy._links,
        templates: copy._templates,
    }
}

fn extend_comment(mut comment: Comment) -> Comment {
    match comment.id {
        Some(ref id) => {
            let mut links: Vec<Link> = Vec::new();
            links.push(Link { key: "event".to_string(), href: format!("/events/{}", &comment.event_id) });
            links.push(Link { key: "self".to_string(), href: format!("/events/{}/comments/{}", &comment.event_id, &id) });
            comment._links = Some(links);

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
                properties: Some(get_comment_properties(&comment.event_id)),
                target: Some("self".to_string()),
            });
            comment._templates = Some(templates);

            comment
        },
        None => comment
    }
}

fn get_comment_properties(event_id: &str) -> Vec<Property> {
    let mut properties: Vec<Property> = Vec::new();
    properties.push(Property {
        name: "eventId".to_string(), prompt: None, read_only: true, required: true, templated: None, value: Some(event_id.to_string())
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

fn comment_links(event_id: &str) -> Vec<Link> {
    let mut links: Vec<Link> = Vec::new();
    links.push(Link {
        key: "self".to_string(),
        href: format!("/events/{}/comments", &event_id),
    });
    links
}

fn comment_templates() -> Vec<Template> {
    let mut templates: Vec<Template> = Vec::new();
    templates.push(Template {
        key: "default".to_string(),
        title: Some("Create comment".to_string()),
        method: MethodType::POST,
        properties: Some(get_create_properties()),
        target: Some("self".to_string())
    });
    templates
}

