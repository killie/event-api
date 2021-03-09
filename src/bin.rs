#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::json::{Json, JsonValue};
use rocket::http::RawStr;

use lib::db::{EventDb, file_based::FileBasedEventDb};
use lib::model::{self, Event, EventFilter, Comment, CommentFilter};
use lib::envelope::{self, Envelope};

fn main() {
    rocket().launch();
}

fn get_event_db() -> impl EventDb {
    FileBasedEventDb {}
}

#[get("/?<from>&<to>&<appName>")]
fn get_events(from: Option<i64>, to: Option<i64>, appName: Option<String>) -> Envelope {
    let edb = get_event_db();
    
    if from.is_none() && to.is_none() && appName.is_none() {
        return match edb.get_events(None) {
            Ok(events) => envelope::success(model::get_events_payload(events)),
            Err(err) => envelope::error(1, "what".to_string()),
        }
    }
    
    let filter = EventFilter {
        from: from,
        to: to,
        app_name: appName,
        source_id: None,
        source_name: None
    };
    
    match edb.get_events(Some(filter)) {
        Ok(events) => envelope::success(model::get_events_payload(events)),
        Err(err) => envelope::error(1, "noo".to_string()),
    }
}

#[post("/", data="<event>")]
fn create_event(event: Json<Event>) -> Envelope {
    match get_event_db().create_event(event.0) {
        Ok(event) => envelope::success(model::get_event_payload(event)),
        Err(err) => envelope::error(2, "no can do".to_string()),
    }
}

#[get("/<id>")]
fn get_event(id: &RawStr) -> Envelope {
    let id_string = id.url_decode().expect("Failed to decode event ID.");
    match get_event_db().get_event(id_string) {
        Ok(event) => envelope::success(model::get_event_payload(event)),
        Err(err) => envelope::error(3, "uh-oh".to_string()),
    }
}    

#[get("/<id>/comments")]  
fn get_comments(id: &RawStr) -> Envelope {
    let id_string = id.url_decode().expect("Failed to decode event ID.");
    let id_copy = id_string.clone();
    let filter = CommentFilter { event_id: Some(id_string), user_id: None };
    match get_event_db().get_comments(Some(filter)) {
        Ok(comments) => envelope::success(model::get_comments_payload(id_copy, comments)),
        Err(err) => envelope::error(4, "huh".to_string()),
    }
}

#[post("/<_id>/comments", data="<comment>")]
fn create_comment(_id: &RawStr, comment: Json<Comment>) -> Envelope {
    match get_event_db().create_comment(comment.0) {
        Ok(comment) => envelope::success(model::get_comment_payload(comment)),
        Err(err) => envelope::error(5, "oh no".to_string()),
    }
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount(
        "/events",
        routes![get_events, get_event, create_event, get_comments, create_comment],
    )
}
