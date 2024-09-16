use rocket::serde::Serialize;
use std::collections::BTreeMap;
use domain::models::{BlogPost, Tour};
use domain::structs::TourLeaderboardDivision;

#[derive(Serialize)]
pub struct TourLeaderboard {
    pub tour: Tour,
    pub divisions: BTreeMap<String, TourLeaderboardDivision>,
}

#[derive(Serialize)]
pub enum ResponseBody {
    Message(String),
    BlogPost(BlogPost),
    BlogPosts(Vec<BlogPost>),
    Tour(Tour),
    Tours(Vec<Tour>),
    TourLeaderboard(TourLeaderboard),
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response<T> {
    pub status: String,
    pub body: Option<T>,
    pub error: Option<String>,
}

impl<T> Response<T> {
    pub fn success(body: T) -> Self {
        Response {
            status: "success".into(),
            body: Some(body),
            error: None,
        }
    }

    pub fn error(message: String) -> Self {
        Response {
            status: "error".into(),
            body: None,
            error: Some(message),
        }
    }
}
