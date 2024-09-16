use rocket::{Request, Response as RocketResponse, http::Status};
use rocket::response::Responder;
use rocket::serde::json::Json;
use application::errors::ApplicationError;
use shared::response_models::Response;

pub struct ApiError(pub ApplicationError);

impl<'r> Responder<'r, 'r> for ApiError {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'r> {
        let ApiError(app_error) = self;

        let (status, error_message) = match app_error {
            ApplicationError::DatabaseError(msg) => (Status::InternalServerError, msg),
            ApplicationError::NotFound(msg) => (Status::NotFound, msg),
        };

        let error_response: Json<Response<()>> = Json(Response::error(error_message));
        let response_body = match serde_json::to_string(&error_response.into_inner()) {
            Ok(body) => body,
            Err(_) => return Err(Status::InternalServerError),
        };

        RocketResponse::build()
            .status(status)
            .header(rocket::http::ContentType::JSON)
            .sized_body(response_body.len(), std::io::Cursor::new(response_body))
            .ok()
    }
}

