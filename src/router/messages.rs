use actix_session::Session;
use actix_web::web::{Json, Query};
use actix_web::{get, post, web, HttpResponse};
use application::dto::messages::{ListMessagesFromIntervalParams, SendMessageDto};
use domain::value_objects::{MessageId, UserId};
use errors::Error;

use crate::state::AppState;

#[get("")]
pub async fn list_messages(
    state: web::Data<AppState>,
    params: Query<ListMessagesFromIntervalParams>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let result = state
        .message_service
        .find_from_interval(
            params.into_inner(),
            session.get::<UserId>("user_id").unwrap().unwrap(),
        )
        .await?;

    Ok(HttpResponse::Ok().json(result))
}

#[get("{id}")]
pub async fn get_message_by_id(
    state: web::Data<AppState>,
    id: web::Path<String>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let result = state
        .message_service
        .get_by_id(
            MessageId(id.into_inner()),
            session.get("user_id").unwrap().unwrap(),
        )
        .await?;

    Ok(HttpResponse::Ok().json(result))
}

#[post("")]
pub async fn send_message(
    state: web::Data<AppState>,
    body: Json<SendMessageDto>,
    session: Session,
) -> Result<HttpResponse, Error> {
    state
        .message_service
        .send_message(
            body.into_inner(),
            session.get::<UserId>("user_id").unwrap().unwrap(),
        )
        .await?;

    Ok(HttpResponse::Created().finish())
}
