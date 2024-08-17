use domain::models::Tour;
use domain::models::NewTour;
use domain::schema::tours::dsl::*;
use infrastructure::establish_connection;

use diesel::ExpressionMethods;
use diesel::insert_into;
use diesel::RunQueryDsl;

pub fn create_tour(tour_data: NewTour) -> Result<(), Box<dyn std::error::Error>> {
    match insert_into(tours).values((title.eq(tour_data.title), start_date.eq(tour_data.start_date), end_date.eq(tour_data.end_date), url.eq(tour_data.url), score_count.eq(tour_data.score_count))).get_result::<Tour>(&mut establish_connection()) {
        Ok(_) => { Ok(()) },
        Err(err) => match err {
            // TODO: Handle specific errors here.
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}
