#[macro_use] extern crate rocket;
use api::blogpost_handler;
use api::event_handler;
use api::tour_handler;

use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

use std::path::PathBuf;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "GET, POST, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type, X-Api-Key"));
    }
}

#[options("/<_path..>")]
fn options(_path: PathBuf) -> &'static str {
    ""
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .mount("/api", routes![
            options, // handle preflight OPTIONS requests
            blogpost_handler::list_blogposts_handler,
            blogpost_handler::list_blogpost_handler,
            event_handler::event_parse_and_save_csv_handler,
            event_handler::event_create,
            event_handler::list,
            tour_handler::tour_create,
            tour_handler::list,
            tour_handler::tour_get,
            tour_handler::leaderboard,
        ])
        .mount("/static", rocket::fs::FileServer::from("static"))
}
