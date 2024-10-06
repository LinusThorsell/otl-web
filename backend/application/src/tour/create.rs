use domain::models::Tour;
use domain::models::NewTour;
use domain::schema::tours::dsl::*;
use infrastructure::establish_connection;

use diesel::insert_into;
use diesel::RunQueryDsl;

use crate::errors::ApplicationError;

pub fn create_tour(tour_data: NewTour) -> Result<(), ApplicationError> {
    match insert_into(tours).values(&tour_data).get_result::<Tour>(&mut establish_connection()) {
        Ok(_) => { Ok(()) }, // TODO: return id
        Err(err) => match err {
            _ => {
                let message = format!("Error creating tour {}", err);
                Err(ApplicationError::DatabaseError(message))
            }
        }
    }
}
