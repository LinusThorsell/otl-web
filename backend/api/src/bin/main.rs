#[macro_use] extern crate rocket;
use api::blogpost_handler;
use api::event_handler;
use api::tour_handler;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![
            blogpost_handler::list_blogposts_handler,
            blogpost_handler::list_blogpost_handler,
            event_handler::event_parse_and_save_csv_handler,
            event_handler::event_create,
            tour_handler::tour_create,
            tour_handler::tour_list,
            tour_handler::tour_get,
            tour_handler::tour_get_leaderboard,
        ])
}
