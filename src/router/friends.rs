use actix_session::Session;
use actix_web::web::Json;
use actix_web::{get, post, web, HttpResponse};
use application::dto::users::FollowUserDto;
use application::services::UsersService;
use domain::value_objects::UserId;
use errors::Error;

use crate::state::AppState;

#[get("")]
pub async fn list_friends(
    state: web::Data<AppState>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let result = UsersService::new(&state.users_repository, &state.password_hasher)
        .list_friends(session.get::<UserId>("user_id").unwrap().unwrap())
        .await?;

    Ok(HttpResponse::Ok().json(result))
}

#[get("count")]
pub async fn count_friends(
    state: web::Data<AppState>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let result = UsersService::new(&state.users_repository, &state.password_hasher)
        .get_friends_count(session.get::<UserId>("user_id").unwrap().unwrap())
        .await?;

    Ok(HttpResponse::Ok().json(result))
}

#[post("")]
pub async fn follow(
    state: web::Data<AppState>,
    body: Json<FollowUserDto>,
    session: Session,
) -> Result<HttpResponse, Error> {
    UsersService::new(&state.users_repository, &state.password_hasher)
        .follow(
            body.into_inner(),
            session.get::<UserId>("user_id").unwrap().unwrap(),
        )
        .await?;

    Ok(HttpResponse::Created().finish())
}
