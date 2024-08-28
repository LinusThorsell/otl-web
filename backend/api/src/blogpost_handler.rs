use shared::response_models::Response;
use application::blogpost::read;
use domain::models::BlogPost;
use rocket::{get, serde::json::Json};

use crate::errors::ApiError;

#[get("/blogposts")]
pub fn list_blogposts_handler() -> Result<Json<Response<Vec<BlogPost>>>, ApiError> {
    let blogposts: Vec<BlogPost> = read::list_blogposts();
    Ok(Json(Response::success(blogposts)))
}

#[get("/blogpost/<blogpost_id>")]
pub fn list_blogpost_handler(blogpost_id: i32) -> Result<Json<Response<BlogPost>>, ApiError> {
    let blogpost = read::list_blogpost(blogpost_id).map_err(ApiError)?;
    Ok(Json(Response::success(blogpost)))
}
