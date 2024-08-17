use serde::Serialize;

use crate::models::{Score, User};

#[derive(Serialize)]
pub struct TourLeaderboardEntry {
    pub user: User,
    pub scores: Vec<Score>,
    pub total_score: i32,
    pub division: String
}
