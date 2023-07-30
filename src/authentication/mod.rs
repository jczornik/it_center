pub mod user;

use crate::get_connection;
use crate::DbPool;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{dev::ServiceRequest, web, Error};
use actix_web::{HttpResponse, ResponseError};
use actix_web_httpauth::extractors::basic::BasicAuth;
use derive_more::{Display, Error};
use diesel::PgConnection;

use self::user::find_user;

#[derive(Debug, Display, Error)]
pub enum SimpleAuthError {
    #[display(fmt = "Cannot authenticate user")]
    CannotAuthenticate,
    #[display(fmt = "Password cannot be empty")]
    EmptyPasswordNotAllowed,
    #[display(fmt = "Internal server error")]
    InternalServerError,
}

impl ResponseError for SimpleAuthError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Self::CannotAuthenticate => StatusCode::UNAUTHORIZED,
            Self::EmptyPasswordNotAllowed => StatusCode::BAD_REQUEST,
            Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub async fn validate(
    pool: DbPool,
    req: ServiceRequest,
    credentials: BasicAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let res: Result<(), SimpleAuthError> = web::block(move || {
        let mut connection = get_connection(pool);
        authenticate(&mut connection, credentials)
    })
    .await
    .unwrap_or(Err(SimpleAuthError::InternalServerError));

    match res {
        Ok(()) => Ok(req),
        Err(e) => Err((Error::from(e), req)),
    }
}

pub fn authenticate(
    pool: &mut PgConnection,
    credentials: BasicAuth,
) -> Result<(), SimpleAuthError> {
    if credentials.password().is_none() || credentials.user_id().is_empty() {
        return Err(SimpleAuthError::EmptyPasswordNotAllowed);
    }

    match find_user(credentials, pool) {
        Ok(_) => Ok(()),
        Err(_) => Err(SimpleAuthError::CannotAuthenticate),
    }
}
