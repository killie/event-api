use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use rocket::request::Request;
use rocket::response::{self, Response, Responder};
use rocket::http::ContentType;
use rocket_contrib::json::{Json, JsonValue};

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    OK,
    Error,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub code: i32,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Envelope {
    pub status: Status,
    pub data: Option<JsonValue>,
    pub error: Option<Error>,
    #[serde(rename = "pageNumber")]
    pub page_number: Option<i32>,
    #[serde(rename = "nextPage")]
    pub next_page: Option<String>,
    #[serde(rename = "totalPages")]
    pub total_pages: Option<i32>,
    pub _links: Option<HashMap<String, Link>>,
    pub _templates: Option<HashMap<String, Template>>,
}

impl<'a> Responder<'a> for Envelope {
    fn respond_to(self, req: &Request) -> response::Result<'a> {
        Response::build_from(Json(self).respond_to(req).unwrap())
            .header(ContentType::JSON)
            .ok()
    }
}

pub struct Payload {
    pub data: JsonValue,
    pub links: Option<Vec<Link>>,
    pub templates: Option<Vec<Template>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub key: String,
    pub href: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MethodType {
    GET,
    POST,
    PATCH,
    DELETE,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub key: String,
    pub method: MethodType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<Property>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    pub read_only: bool,
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub templated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

pub fn error(code: i32, description: String) -> Envelope {
    let error = Error { code, description };
    Envelope {
        status: Status::Error,
        data: None,
        error: Some(error),
        page_number: None,
        next_page: None,
        total_pages: None,
        _links: None,
        _templates: None,
    }
}

pub fn success(payload: Payload) -> Envelope {
    let mut links = HashMap::new();
    if payload.links.is_some() {
        for link in payload.links.unwrap().iter() {
            links.insert(link.key.clone(), link.clone());
        }
    }

    let mut templates = HashMap::new();
    if payload.templates.is_some() {
        for template in payload.templates.unwrap().iter() {
            templates.insert(template.key.clone(), template.clone());
        }
    }

    Envelope {
        status: Status::OK,
        data: Some(payload.data),
        error: None,
        page_number: None,
        next_page: None,
        total_pages: None,
        _links: Some(links.clone()),
        _templates: Some(templates.clone()),
    }
}

pub fn create_property(name: &str, read_only: bool, required: bool) -> Property {
    Property {
        name: name.to_string(),
        prompt: None,
        read_only: read_only,
        required: required,
        templated: None,
        value: None,
    }
}
