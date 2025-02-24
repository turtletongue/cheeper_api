use actix_session::Session;
use actix_web::web::Json;
use actix_web::{post, web, HttpResponse};
use application::dto::authentication::AuthenticateDto;
use errors::Error;
use log::error;

use crate::state::AppState;

#[post("")]
pub async fn authenticate(
    state: web::Data<AppState>,
    body: Json<AuthenticateDto>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let user_id = state
        .authentication_service
        .authenticate(body.into_inner())
        .await?;

    if let Err(error) = session.insert("user_id", user_id) {
        error!("Failed to save user_id to session: {}", error);
        return Err(Error::InternalServerError {});
    }

    Ok(HttpResponse::Created().finish())
}
