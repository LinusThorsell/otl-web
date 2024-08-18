use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::outcome::Outcome::Success;
use diesel::prelude::*;
use domain::schema::apikeys::dsl::*;
use domain::models::ApiKey;

use infrastructure::establish_connection;

pub struct ApiKeyGuard(pub ApiKey);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKeyGuard {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Extract API key from headers
        let api_key_header = request.headers().get_one("X-Api-Key");

        if let Some(api_key_value) = api_key_header {
            // Access database connection from request guard
            let mut connection = establish_connection();

            // Query the database to find the API key
            let key = apikeys
                .filter(apikey.eq(api_key_value))
                .first::<ApiKey>(&mut connection)
                .optional()
                .unwrap();

            if let Some(key) = key {
                // Check if the API key has expired
                if key.expires > chrono::Utc::now().naive_utc() {
                    return Success(ApiKeyGuard(key));
                }
            }
        }

        Outcome::Error((Status::Unauthorized, ()))
    }
}

