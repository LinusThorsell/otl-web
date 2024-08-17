use domain::models::NewEvent;
use rocket::fs::TempFile;
// use shared::response_models::{Response, ResponseBody};
use rocket::post;

use rocket::serde::json::Json;

use application::event::save;
use application::event::create;

#[post("/event/ingest/<event_id>", format = "multipart/form-data", data = "<file>")]
pub async fn event_parse_and_save_csv_handler(file: TempFile<'_>, event_id: i32) -> std::io::Result<()> {
    match save::event_parse_and_save_csv(file, event_id).await {
        Ok(_) => {},
        Err(_) => {},
    };

    Ok(())
}

#[post("/event/create", format = "application/json", data = "<event_data>")]
pub async fn event_create(event_data: Json<NewEvent>) -> std::io::Result<()> {
    let event = event_data.into_inner();
    match create::create_event(event) {
        Ok(_) => {},
        Err(_) => {},
    };

    Ok(())
}
