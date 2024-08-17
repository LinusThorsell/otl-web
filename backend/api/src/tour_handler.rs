use domain::models::NewTour;
use rocket::response::status::NotFound;
use shared::response_models::{Response, ResponseBody};
use rocket::post;
use rocket::get;

use rocket::serde::json::Json;

use application::tour::create;
use application::tour::read;

#[post("/tour/create", format = "application/json", data = "<tour_data>")]
pub fn tour_create(tour_data: Json<NewTour>) -> std::io::Result<()> {
    let tour = tour_data.into_inner();
    match create::create_tour(tour) {
        Ok(_) => {},
        Err(_) => {},
    };

    Ok(())
}

#[get("/tour/list")]
pub fn tour_list() -> String {
    let tours = read::tour_list();
    let response = Response { body: ResponseBody::Tours(tours) };
    serde_json::to_string(&response).unwrap()
}

#[get("/tour/get/<tour_id>")]
pub fn tour_get(tour_id: i32) -> Result<String, NotFound<String>> {
    let tour = read::tour_get(tour_id)?;
    let response = Response { body: ResponseBody::Tour(tour) };

    Ok(serde_json::to_string(&response).unwrap())
}

#[get("/tour/get/leaderboard/<tour_id>")]
pub fn tour_get_leaderboard(tour_id: i32) ->Result<String, NotFound<String>> {
    let leaderboard = read::tour_get_leaderboard(tour_id)?;
    let response = Response { body: ResponseBody::TourLeaderboard(leaderboard) };

    Ok(serde_json::to_string(&response).unwrap())
}
