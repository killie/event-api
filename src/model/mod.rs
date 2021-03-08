use serde::{Deserialize, Serialize};

use crate::envelope::{Template, Link};

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
    #[serde(skip_deserializing)]
    pub _links: Option<Vec<Link>>,
    #[serde(skip_deserializing)]
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

