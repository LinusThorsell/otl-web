use domain::{models::{BlogPost, Tour}, structs::TourLeaderboardDivision};
use rocket::serde::Serialize;
use std::collections::BTreeMap;

#[derive(Serialize)]
pub struct TourLeaderboard {
    pub tour: Tour,
    pub divisions: BTreeMap<String, TourLeaderboardDivision>
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
pub struct Response {
    pub body: ResponseBody,
}
