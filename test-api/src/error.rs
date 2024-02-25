use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

use crate::api::{
    create_user::CreateUserError, get_users::GetUsersError, update_user::UpdateUserError,
};

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    GetUsersError(#[from] GetUsersError),
    #[error(transparent)]
    CreateUserError(#[from] CreateUserError),
    #[error(transparent)]
    UpdateUserError(#[from] UpdateUserError),
    #[error(transparent)]
    StdIOError(#[from] std::io::Error),
    #[error(transparent)]
    ResponseError(actix_web::Error),
}

impl From<actix_web::Error> for Error {
    fn from(err: actix_web::Error) -> Self {
        Error::ResponseError(err)
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(format!("{:?}", self))
    }
}
