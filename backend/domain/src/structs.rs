use serde::Serialize;

use crate::models::{Score, User};

#[derive(Serialize)]
pub struct TourLeaderboardEntry {
    pub user: User,
    pub scores: Vec<Score>,
    pub total_score: i32,
}

#[derive(Serialize)]
pub struct TourLeaderboardDivision {
    pub entries: Vec<TourLeaderboardEntry>
}
