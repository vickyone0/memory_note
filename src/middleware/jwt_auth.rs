use actix_web::{dev::ServiceRequest, Error, HttpMessage, FromRequest, HttpRequest};
use crate::auth::{decode_jwt,Claims};
use futures_util::future::{ready, Ready};


pub struct  AuthenticatedUser(pub Claims);


impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {

        if let Some(auth_header) =req.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if let Some(token) = auth_str.strip_prefix("Bearer ") {
                    if let Ok(data) = decode_jwt(token) {
                        return ready(Ok(AuthenticatedUser(data.claims)));
                    }
                }
            }
        }
        ready(Err(actix_web::error::ErrorUnauthorized("Invalid or missing JWT")))
    }
}