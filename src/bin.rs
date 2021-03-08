#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::json::{Json, JsonValue};
use rocket::http::RawStr;
use lib::db;
use lib::model::{Event, Comment};
use lib::envelope::{self, Envelope};

fn main() {
    rocket().launch();
}

#[get("/?<from>&<to>&<appName>")]
fn get_events(from: Option<i64>, to: Option<i64>, appName: Option<String>) -> Envelope {
    if from.is_none() && to.is_none() && appName.is_none() {
        return envelope::success(json!(db::get_events()), None, None);
    }
    envelope::success(json!(db::query_events(from, to, appName)), None, None)
}

#[post("/", data="<event>")]
fn create_event(event: Json<Event>) -> Envelope {
    envelope::success(json!(db::create_event(event.0)), None, None)
}

#[get("/<id>")]
fn get_event(id: &RawStr) -> Envelope {
    let id_string = id.url_decode().expect("Failed to decode event ID.");
    envelope::success(json!(db::get_event(&id_string)), None, None)
}    

#[get("/<id>/comments")]  
fn get_comments(id: &RawStr) -> Envelope {
    let id_string = id.url_decode().expect("Failed to decode event ID.");
    envelope::success(json!(db::get_comments(&id_string)), None, None)
}

#[post("/<_id>/comments", data="<comment>")]
fn create_comment(_id: &RawStr, comment: Json<Comment>) -> Envelope {
    envelope::success(json!(db::create_comment(comment.0)), None, None)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount(
        "/events",
        routes![get_events, get_event, create_event, get_comments, create_comment],
    )
}
