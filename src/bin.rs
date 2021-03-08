#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::json::Json;
use rocket::http::RawStr;
use lib::db;
use lib::model::{Event, Comment};

type EventsJson = Json<Option<Vec<Event>>>;
type CommentsJson = Json<Option<Vec<Comment>>>;

fn main() {
    rocket().launch();
}

#[get("/?<from>&<to>&<appName>")]
fn get_events(from: Option<i64>, to: Option<i64>, appName: Option<String>) -> EventsJson {
    if from.is_none() && to.is_none() && appName.is_none() {
        return Json(db::get_events());
    }
    Json(db::query_events(from, to, appName))
}

#[post("/", data="<event>")]
fn create_event(event: Json<Event>) -> Json<Option<Event>> {
    Json(db::create_event(event.0))
}

#[get("/<id>")]
fn get_event(id: &RawStr) -> EventsJson {
    let id_string = id.url_decode().expect("Failed to decode event ID.");
    Json(db::get_event(&id_string))
}    

#[get("/<id>/comments")]  
fn get_comments(id: &RawStr) -> CommentsJson {
    let id_string = id.url_decode().expect("Failed to decode event ID.");
    Json(db::get_comments(&id_string))
}

#[post("/<_id>/comments", data="<comment>")]
fn create_comment(_id: &RawStr, comment: Json<Comment>) -> Json<Option<Comment>> {
    Json(db::create_comment(comment.0))
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount(
        "/events",
        routes![get_events, get_event, create_event, get_comments, create_comment],
    )
}
