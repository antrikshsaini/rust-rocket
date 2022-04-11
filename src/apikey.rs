use rocket::{http::Status, request, request::FromRequest, request::Outcome, Request};
use std::str;

// refer request guards at https://rocket.rs/v0.4/guide/requests/#request-guards

#[derive(Debug)]
pub struct ApiKey(pub String);

#[derive(Debug)]
pub enum ApiKeyError {
    MissingKey,
    InvalidKey,
}

const API_KEY: &str = "secretkey";

fn is_valid(key: &str) -> bool {
    key == API_KEY
}

// FromRequest trait for ApiKey
//  refer  https://api.rocket.rs/v0.4/rocket/request/trait.FromRequest.html

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ApiKeyError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // To Check if API key is in header
        match request.headers().get_one("x-api-key") {
            Some(s) => {
                if is_valid(s) {
                    Outcome::Success(ApiKey(s.to_string()))
                } else {
                    Outcome::Failure((Status::Unauthorized, ApiKeyError::InvalidKey))
                }
            },
            None => Outcome::Failure((Status::Unauthorized, ApiKeyError::MissingKey)),
        }
    }
}