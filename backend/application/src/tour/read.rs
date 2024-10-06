use core::panic;
use std::collections::{BTreeMap, HashMap};

use domain::{models::{Event, Score, Tour, User}, schema::{events, scores, users}, structs::{TourLeaderboardEntry, TourLeaderboardDivision}};
use shared::response_models::TourLeaderboard;
use infrastructure::establish_connection;
use diesel::prelude::*;

use crate::errors::ApplicationError;

pub fn tour_list() -> Result<Vec<Tour>, ApplicationError> {
    use domain::schema::tours;

    match tours::table.select(tours::all_columns).load::<Tour>(&mut establish_connection()) {
        Ok(mut tours) => {
            tours.sort();
            Ok(tours)
        },
        Err(err) => match err {
            _ => {
                let message = format!("Error listing tours {}", err);
                Err(ApplicationError::DatabaseError(message))
            }
        }
    }
}

pub fn tour_get(tour_id: i32) -> Result<Tour, ApplicationError> {
    use domain::schema::tours;

    match tours::table.find(tour_id).first::<Tour>(&mut establish_connection()) {
        Ok(tour) => Ok(tour),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let message = format!("Error finding tour with id {} - {}", tour_id, err);
                Err(ApplicationError::NotFound(message))
            },
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}

pub fn tour_get_leaderboard(tour_id: i32) -> Result<TourLeaderboard, ApplicationError> {
    // Fetch the tour object by ID
    let tour_obj = tour_get(tour_id)?;

    // Establish database connection
    let connection = &mut establish_connection();

    // Fetch all events associated with the tour
    let tour_events = events::table
        .filter(events::tour_id.eq(Some(tour_obj.id)))
        .load::<Event>(connection)
        .map_err(|err| panic!("Database error - {}", err))?;

    // Extract event IDs
    let event_ids: Vec<i32> = tour_events.iter().map(|event| event.id).collect();

    // Fetch all scores associated with these events
    let scores = scores::table
        .filter(scores::event_id.eq_any(event_ids))
        .load::<Score>(connection)
        .map_err(|err| panic!("Database error - {}", err))?;

    // Extract unique user IDs from the scores
    let user_ids: Vec<i32> = scores.iter().map(|score| score.user_id).collect();

    // Fetch users associated with these user IDs
    let users = users::table
        .filter(users::id.eq_any(user_ids))
        .load::<User>(connection)
        .map_err(|err| panic!("Database error - {}", err))?;

    // Create a map of user ID to User object for easy lookup
    let user_map: HashMap<i32, User> = users.into_iter().map(|user| (user.id, user)).collect();

    // Group scores by user ID
    let mut user_scores: HashMap<i32, Vec<Score>> = HashMap::new();
    for score in scores {
        user_scores
            .entry(score.user_id)
            .or_insert_with(Vec::new)
            .push(score);
    }

    // Map to store division to TourLeaderboardDivision
    let mut division_map: BTreeMap<String, TourLeaderboardDivision> = BTreeMap::new();

    // Iterate over user scores to create leaderboard entries
    for (user_id, scores) in user_scores {
        if let Some(user) = user_map.get(&user_id) {
            // Sort scores in descending order
            let mut sorted_scores = scores.clone();
            sorted_scores.sort_by(|a, b| b.score.cmp(&a.score));

            // Take the top N scores based on the tour's score_count
            let top_scores: Vec<Score> = sorted_scores
                .into_iter()
                .take(tour_obj.score_count.max(0) as usize)
                .collect();

            // Sum the top scores to get the total score
            let total_score: i32 = top_scores.iter().map(|score| score.score).sum();

            // Determine the division for the user (from the last score's divcode)
            let division = scores
                .last()
                .map_or("Unknown".to_string(), |s| s.divcode.clone());

            // Create a leaderboard entry for this user
            let entry = TourLeaderboardEntry {
                user: user.clone(),
                scores: top_scores,
                total_score,
                placement: 0,
            };

            // Add this entry to the appropriate division in the map
            division_map
                .entry(division.clone())
                .or_insert_with(|| TourLeaderboardDivision {
                    entries: Vec::new(),
                })
                .entries
                .push(entry);
        }
    }

    // Sort each division's entries by total score and tie-breakers, and assign placements
    for division in division_map.values_mut() {
        // Sort entries using the custom comparator
        division.entries.sort_by(|a, b| {
            // First compare total_score in descending order
            let cmp_total = b.total_score.cmp(&a.total_score);
            if cmp_total != std::cmp::Ordering::Equal {
                return cmp_total;
            }
            // Compare individual scores for tie-breaking
            let min_len = std::cmp::min(a.scores.len(), b.scores.len());
            for i in 0..min_len {
                let cmp_score = b.scores[i].score.cmp(&a.scores[i].score);
                if cmp_score != std::cmp::Ordering::Equal {
                    return cmp_score;
                }
            }
            // If all compared scores are equal, the player with more scores is better
            let cmp_len = b.scores.len().cmp(&a.scores.len());
            if cmp_len != std::cmp::Ordering::Equal {
                return cmp_len;
            }
            // As a last resort, compare user IDs to ensure consistent ordering
            a.user.id.cmp(&b.user.id)
        });

        // Assign placements
        let mut current_placement = 1;
        let mut previous_entry: Option<&TourLeaderboardEntry> = None;

        for (i, entry) in division.entries.iter_mut().enumerate() {
            if let Some(prev) = previous_entry {
                // Check if the current entry ties with the previous entry
                let mut is_tie = entry.total_score == prev.total_score;
                if is_tie {
                    let min_len = std::cmp::min(entry.scores.len(), prev.scores.len());
                    for j in 0..min_len {
                        if entry.scores[j].score != prev.scores[j].score {
                            is_tie = false;
                            break;
                        }
                    }
                    if is_tie && entry.scores.len() != prev.scores.len() {
                        is_tie = false;
                    }
                }
                if !is_tie {
                    current_placement = i + 1;
                }
            } else {
                current_placement = 1;
            }
            entry.placement = current_placement;
            previous_entry = Some(entry);
        }
    }

    // Create the final leaderboard object with a map of divisions
    let leaderboard = TourLeaderboard {
        tour: tour_obj,
        divisions: division_map,
    };

    Ok(leaderboard)
}

