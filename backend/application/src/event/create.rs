use domain::models::Event;
use domain::models::NewEvent;
use domain::schema::events::dsl::*;
use infrastructure::establish_connection;

use diesel::ExpressionMethods;
use diesel::insert_into;
use diesel::RunQueryDsl;

pub fn create_event(event_data: NewEvent) -> Result<(), Box<dyn std::error::Error>> {
    match insert_into(events)
        .values((title.eq(event_data.title), date.eq(event_data.date), url.eq(event_data.url), image.eq(event_data.image), tour_id.eq(event_data.tour_id)))
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
