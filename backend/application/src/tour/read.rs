use core::panic;
use std::collections::HashMap;

use domain::{models::{Event, Score, Tour, User}, schema::{events, scores, users}, structs::TourLeaderboardEntry};
use shared::response_models::{Response, ResponseBody, TourLeaderboard};
use infrastructure::establish_connection;
use diesel::prelude::*;
use rocket::response::status::NotFound;

pub fn tour_list() -> Vec<Tour> {
    use domain::schema::tours;

    match tours::table.select(tours::all_columns).load::<Tour>(&mut establish_connection()) {
        Ok(mut tours) => {
            tours.sort();
            tours
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}

pub fn tour_get(tour_id: i32) -> Result<Tour, NotFound<String>> {
    use domain::schema::tours;

    match tours::table.find(tour_id).first::<Tour>(&mut establish_connection()) {
        Ok(tour) => Ok(tour),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error finding tour with id {} - {}", tour_id, err)) };
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}

pub fn tour_get_leaderboard(tour_id: i32) -> Result<TourLeaderboard, NotFound<String>> {
    let tour_obj = tour_get(tour_id)?;

    let connection = &mut establish_connection();

    let tour_events = events::table.filter(events::tour_id.eq(Some(tour_obj.id))).load::<Event>(connection).map_err(|err| panic!("Database error - {}", err))?;

    let event_ids: Vec<i32> = tour_events.iter().map(|event| event.id).collect();
    let scores = scores::table.filter(scores::event_id.eq_any(event_ids)).load::<Score>(connection).map_err(|err| panic!("Database error - {}", err))?;

    let user_ids: Vec<i32> = scores.iter().map(|score| score.user_id).collect();
    let users = users::table.filter(users::id.eq_any(user_ids)).load::<User>(connection).map_err(|err| panic!("Database error - {}", err))?;

    let user_map: HashMap<i32, User> = users.into_iter().map(|user| (user.id, user)).collect();

    let mut user_scores: HashMap<i32, Vec<Score>> = HashMap::new();
    for score in scores {
        user_scores.entry(score.user_id).or_insert_with(Vec::new).push(score);
    }

    let mut leaderboard_entries: Vec<TourLeaderboardEntry> = Vec::new();
    for (user_id, scores) in user_scores {
        if let Some(user) = user_map.get(&user_id) {
            let mut sorted_scores = scores.clone();
            sorted_scores.sort_by(|a, b| b.score.cmp(&a.score));
            let top_scores = sorted_scores.into_iter().take(tour_obj.score_count.max(0) as usize);
            let total_score: i32 = top_scores.map(|score| score.score).sum();
            let division = scores.last().map_or("Unknown".to_string(), |s| s.divcode.clone());

            leaderboard_entries.push(TourLeaderboardEntry {
                user: user.clone(),
                total_score,
                scores,
                division
            });
        }
    }

    leaderboard_entries.sort_by(|a, b| b.total_score.cmp(&a.total_score));
    let leaderboard = TourLeaderboard {
        entries: leaderboard_entries,
        tour: tour_obj
    };

    Ok(leaderboard)
}
