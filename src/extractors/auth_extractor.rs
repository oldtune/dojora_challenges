use std::{
    collections::HashSet,
    future::{self, Ready},
};

use actix_web::FromRequest;
use jsonwebtoken::{errors::ErrorKind, Algorithm, DecodingKey, Validation};

use crate::routes::{auth_route::Claims, route_error::ApplicationError};

//contains username
pub struct AuthorisedUser {
    pub username: String,
    pub id: uuid::Uuid,
}

impl FromRequest for AuthorisedUser {
    type Error = ApplicationError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let auth_header = req.headers().get("Authorization");
        if let Some(auth_header_value) = auth_header {
            let token = auth_header_value.to_str().unwrap();
            let token = &token.replace("Bearer ", "");

            let mut validation = Validation::new(Algorithm::HS256);
            validation.validate_exp = false;
            validation.required_spec_claims = HashSet::new();

            let claims = jsonwebtoken::decode::<Claims>(
                token,
                &DecodingKey::from_secret(b"MeowMeowMeowMeowMeowMeowMeowMeow"),
                &validation,
            );

            if claims.is_err() {
                println!("{:?}", claims);
                return future::ready(Err(ApplicationError::Authorization));
            }

            let claims = claims.unwrap();
            return future::ready(Ok(AuthorisedUser {
                username: claims.claims.username,
                id: claims.claims.id,
            }));
        }

        return future::ready(Err(ApplicationError::Authorization));
    }
}
