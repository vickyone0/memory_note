use actix_web::{FromRequest};
use crate::auth::{decode_jwt,Claims};


pub struct  AuthenticatedUser(pub Claims);


impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request
}