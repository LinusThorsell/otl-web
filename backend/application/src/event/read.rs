use domain::models::Event;
use infrastructure::establish_connection;
use diesel::prelude::*;

use crate::errors::ApplicationError;

pub fn list_events() -> Result<Vec<Event>, ApplicationError> {
    use domain::schema::events;

    match events::table.select(events::all_columns).load::<Event>(&mut establish_connection()) {
        Ok(mut events) => {
            events.sort();
            Ok(events)
        },
        Err(err) => match err {
            _ => {
                return Err(ApplicationError::DatabaseError(err.to_string()));
            }
        }
    }
}
