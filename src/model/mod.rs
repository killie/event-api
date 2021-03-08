use serde::{Deserialize, Serialize};

use crate::envelope::{Template, Link};

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: Option<String>,
    pub from: i64,
    pub to: Option<i64>,
    pub text: String,
    pub app_name: Option<String>,
    pub source_id: Option<String>,
    pub source_name: Option<String>,
    pub _links: Option<Vec<Link>>,
    pub _templates: Option<Vec<Template>>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: Option<String>,
    pub event_id: String,
    pub user_id: String,
    pub comment: String,
    pub timestamp: i64,
    pub _links: Option<Vec<Link>>,
    pub _templates: Option<Vec<Template>>,
}

/*
impl Data for Event {
    fn get_links(&self) -> Vec<Link> {
        self._links.unwrap_or(vec![])
    }

    fn get_templates(&self) -> Vec<Template> {
        self._templates.unwrap_or(vec![])
    }
}

impl Data for Comment {
    fn get_links(&self) -> Vec<Link> {
        self._links.unwrap_or(vec![])
    }

    fn get_templates(&self) -> Vec<Template> {
        self._templates.unwrap_or(vec![])
    }
}
*/
