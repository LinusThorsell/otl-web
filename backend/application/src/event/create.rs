use domain::models::Event;
use domain::models::NewEvent;
use domain::schema::events::dsl::*;
use infrastructure::establish_connection;

use diesel::insert_into;
use diesel::RunQueryDsl;

pub fn create_event(event_data: NewEvent) -> Result<(), Box<dyn std::error::Error>> {
    match insert_into(events)
        .values(&event_data)
        .get_result::<Event>(&mut establish_connection()) {
        Ok(_) => { Ok(()) },
        Err(err) => match err {
            // TODO: Handle specific errors here.
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}
