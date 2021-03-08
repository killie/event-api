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
    pub _links: Option<Vec<Link>>,
    pub _templates: Option<Vec<Template>>,
}

impl<'a> Responder<'a> for Envelope {
    fn respond_to(self, req: &Request) -> response::Result<'a> {
        Response::build_from(Json(self).respond_to(req).unwrap())
            .header(ContentType::JSON)
            .ok()
    }
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

pub fn success(data: JsonValue, links: Option<Vec<Link>>, templates: Option<Vec<Template>>) -> Envelope {
    Envelope {
        status: Status::OK,
        data: Some(data),
        error: None,
        page_number: None,
        next_page: None,
        total_pages: None,
        _links: links,
        _templates: templates,
    }
}

