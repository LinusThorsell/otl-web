use domain::models::Tour;
use domain::models::NewTour;
use domain::schema::tours::dsl::*;
use infrastructure::establish_connection;

use diesel::insert_into;
use diesel::RunQueryDsl;

pub fn create_tour(tour_data: NewTour) -> Result<(), Box<dyn std::error::Error>> {
    match insert_into(tours).values(&tour_data).get_result::<Tour>(&mut establish_connection()) {
        Ok(_) => { Ok(()) }, // TODO: return id
        Err(err) => match err {
            // TODO: Handle specific errors here.
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}
