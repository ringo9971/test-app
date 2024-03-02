use crate::error::Error;
use actix_web::{dev::Payload, FromRequest, HttpRequest};
use futures::future::{ready, Ready};
use serde::Deserialize;
use thiserror::Error;

use crate::config::AuthConfig;

#[derive(Error, Debug)]
pub enum PermissionError {
    #[error("Permission denied")]
    PermissionDenied,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Token {
    pub user_uid: String,
}

impl FromRequest for Token {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if let Some(token_str) = auth_str.strip_prefix("Bearer ") {
                    return ready(Ok(Token {
                        user_uid: token_str.to_string(),
                    }));
                }
            }
        }
        ready(Err(Error::PermissionError(
            PermissionError::PermissionDenied,
        )))
    }
}

pub fn check_permission(auth_config: &AuthConfig, token: &Token) -> Result<(), Error> {
    let alloweb_user_uid = auth_config.allowed_user_uids.to_owned();

    if alloweb_user_uid.contains(&token.user_uid) {
        return Ok(());
    }

    Err(PermissionError::PermissionDenied.into())
}
