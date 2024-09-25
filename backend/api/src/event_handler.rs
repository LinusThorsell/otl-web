use rocket::FromForm;
use std::path::Path;
use rocket::form::Form;
use application::auth::apikey_guard::ApiKeyGuard;
use domain::models::Event;
use domain::models::NewEvent;
use rocket::fs::TempFile;
// use shared::response_models::{Response, ResponseBody};
use rocket::post;
use rocket::get;

use rocket::serde::json::Json;

use application::event::save;
use application::event::create;
use application::event::read;
use shared::response_models::Response;

use crate::errors::ApiError;

#[post("/event/ingest/<event_id>", format = "multipart/form-data", data = "<file>")]
pub async fn event_parse_and_save_csv_handler(file: TempFile<'_>, event_id: i32, _apikey: ApiKeyGuard) -> Result<Json<Response<()>>, ApiError> {
    match save::event_parse_and_save_csv(file, event_id).await {
        Ok(_) => {},
        Err(_) => {},
    };

    Ok(Json(Response::success(())))
}


#[derive(FromForm)]
pub struct EventForm<'r> {
    pub title: String,
    pub date: String,
    pub url: String,
    pub tour_id: Option<i32>,
    pub image: TempFile<'r>,
}

#[post("/event/create", format = "multipart/form-data", data = "<event_data>")]
pub async fn event_create(mut event_data: Form<EventForm<'_>>, _apikey: ApiKeyGuard) -> Result<Json<Response<()>>, ApiError> {
    let image_name = format!("{}.png", uuid::Uuid::new_v4());
    let save_path = Path::new("static/uploads/").join(&image_name);

    if let Err(e) = event_data.image.copy_to(&save_path).await {
        eprintln!("Failed to save image: {}", e);
    }

    let event = NewEvent {
        title: event_data.title.clone(),
        date: chrono::NaiveDateTime::parse_from_str(&event_data.date, "%Y-%m-%d %H:%M:%S").unwrap(),
        url: event_data.url.clone(),
        tour_id: event_data.tour_id,
        image: image_name,
    };

    match create::create_event(event) {
        Ok(_) => {},
        Err(_) => {},
    };

    Ok(Json(Response::success(())))
}

#[get("/events")]
pub fn list() -> Result<Json<Response<Vec<Event>>>, ApiError> {
    let events: Vec<Event> = read::list_events().map_err(ApiError)?;
    Ok(Json(Response::success(events)))
}
