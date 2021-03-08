use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub enum Status {
    OK,
    Error,
}

#[derive(Serialize, Deserialize)]
pub struct Error {
    pub code: i32,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct Envelope {
    pub status: Status,
    pub data: Option<Value>,
    pub error: Option<Error>,
    pub page_number: Option<i32>,
    pub next_page: Option<String>,
    pub total_pages: Option<i32>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Link {
    pub key: String,
    pub href: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum MethodType {
    GET,
    POST,
    PATCH,
    DELETE,
}

#[derive(Clone, Serialize, Deserialize)]
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

#[derive(Clone, Serialize, Deserialize)]
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

/*
pub trait Data: Serialize + 'Deserialize {
    fn get_links(&self) -> Vec<Link>;
    fn get_templates(&self) -> Vec<Template>;
}
*/

pub fn error(code: i32, description: String) -> Envelope {
    let error = Error { code, description };
    Envelope {
        status: Status::Error,
        data: None,
        error: Some(error),
        page_number: None,
        next_page: None,
        total_pages: None
    }
}

pub fn success(data: Value) -> Envelope {
    Envelope {
        status: Status::OK,
        data: Some(data),
        error: None,
        page_number: None,
        next_page: None,
        total_pages: None
    }
}

