use rocket::FromForm;
use std::path::Path;
use rocket::form::Form;
use rocket::fs::TempFile;
use domain::models::NewTour;
use domain::models::Tour;
use shared::response_models::TourLeaderboard;
use shared::response_models::Response;
use rocket::post;
use rocket::get;

use rocket::serde::json::Json;

use application::tour::create;
use application::tour::read;
use application::auth::apikey_guard::ApiKeyGuard;

use crate::errors::ApiError;

#[derive(FromForm)]
pub struct TourForm<'r> {
    pub title: String,
    pub location: String,
    pub description: String,
    pub body: String,
    pub start_date: String,
    pub end_date: String,
    pub url: String,
    pub score_count: i32,
    pub image: TempFile<'r>,
}

#[post("/tour", format = "multipart/form-data", data = "<tour_data>")]
pub async fn tour_create(mut tour_data: Form<TourForm<'_>>, _apikey: ApiKeyGuard) -> Result<Json<Response<()>>, ApiError> {
    let image_name = format!("{}.png", uuid::Uuid::new_v4());
    let save_path = Path::new("static/uploads/").join(&image_name);

    if let Err(e) = tour_data.image.copy_to(&save_path).await {
        eprintln!("Failed to save image: {}", e);
    }

    let tour = NewTour {
        title: tour_data.title.clone(),
        location: tour_data.location.clone(),
        description: tour_data.description.clone(),
        body: tour_data.body.clone(),
        start_date: chrono::NaiveDateTime::parse_from_str(&tour_data.start_date, "%Y-%m-%dT%H:%M").unwrap(),
        end_date: chrono::NaiveDateTime::parse_from_str(&tour_data.start_date, "%Y-%m-%dT%H:%M").unwrap(),
        url: tour_data.url.clone(),
        score_count: tour_data.score_count,
        image: image_name,
    };

    match create::create_tour(tour) {
        Ok(_) => {},
        Err(_) => {},
    };

    Ok(Json(Response::success(())))
}

#[get("/tours")]
pub fn list() -> Result<Json<Response<Vec<Tour>>>, ApiError> {
    let tours = read::tour_list();
    Ok(Json(Response::success(tours)))
}

#[get("/tour/get/<tour_id>")]
pub fn tour_get(tour_id: i32) -> Result<Json<Response<Tour>>, ApiError> {
    let tour = read::tour_get(tour_id).map_err(ApiError)?;
    Ok(Json(Response::success(tour)))
}

#[get("/tour/leaderboard/<tour_id>")]
pub fn leaderboard(tour_id: i32) -> Result<Json<Response<TourLeaderboard>>, ApiError> {
    let leaderboard = read::tour_get_leaderboard(tour_id).map_err(ApiError)?;
    Ok(Json(Response::success(leaderboard)))
}
