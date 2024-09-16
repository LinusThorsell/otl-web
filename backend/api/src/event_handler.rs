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

#[post("/event/create", format = "application/json", data = "<event_data>")]
pub async fn event_create(event_data: Json<NewEvent>, _apikey: ApiKeyGuard) -> Result<Json<Response<()>>, ApiError> {
    let event = event_data.into_inner();
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
