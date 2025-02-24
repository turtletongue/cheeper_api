use actix_session::Session;
use actix_web::web::Json;
use actix_web::{get, post, web, HttpResponse};
use application::dto::users::CreateUserDto;
use domain::value_objects::UserId;
use errors::Error;

use crate::state::AppState;

#[get("")]
pub async fn list_users(
    state: web::Data<AppState>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let result = state
        .users_service
        .list_users(session.get::<UserId>("user_id").unwrap().unwrap())
        .await?;

    Ok(HttpResponse::Ok().json(result))
}

#[post("")]
pub async fn create(
    state: web::Data<AppState>,
    body: Json<CreateUserDto>,
) -> Result<HttpResponse, Error> {
    state.users_service.add_user(body.into_inner()).await?;

    Ok(HttpResponse::Created().finish())
}
