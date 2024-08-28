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

#[post("/tour/create", format = "application/json", data = "<tour_data>")]
pub fn tour_create(tour_data: Json<NewTour>, _apikey: ApiKeyGuard) -> Result<Json<Response<()>>, ApiError> {
    let tour = tour_data.into_inner();
    match create::create_tour(tour) {
        Ok(_) => {},
        Err(_) => {},
    };

    Ok(Json(Response::success(())))
}

#[get("/tour/list")]
pub fn tour_list() -> Result<Json<Response<Vec<Tour>>>, ApiError> {
    let tours = read::tour_list();
    Ok(Json(Response::success(tours)))
}

#[get("/tour/get/<tour_id>")]
pub fn tour_get(tour_id: i32) -> Result<Json<Response<Tour>>, ApiError> {
    let tour = read::tour_get(tour_id).map_err(ApiError)?;
    Ok(Json(Response::success(tour)))
}

#[get("/tour/get/leaderboard/<tour_id>")]
pub fn tour_get_leaderboard(tour_id: i32) -> Result<Json<Response<TourLeaderboard>>, ApiError> {
    let leaderboard = read::tour_get_leaderboard(tour_id).map_err(ApiError)?;
    Ok(Json(Response::success(leaderboard)))
}
