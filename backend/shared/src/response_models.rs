use domain::{models::{BlogPost, Tour}, structs::TourLeaderboardEntry};
use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct TourLeaderboard {
    pub entries: Vec<TourLeaderboardEntry>,
    pub tour: Tour,
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
