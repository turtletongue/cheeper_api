use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::Key;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use cheeper::config::Config;
use cheeper::guards::AuthGuard;
use cheeper::router;
use cheeper::state::AppState;
use mongodb::Client;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::build();

    let mongo_client = Arc::new(Client::with_uri_str(config.mongo_connection).await.unwrap());

    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    Key::from(config.session_key.as_bytes()),
                )
                .cookie_secure(false)
                .build(),
            )
            .app_data(web::Data::new(AppState::new(mongo_client.clone())))
            .service(web::scope("/authentication").service(router::authentication::authenticate))
            .service(
                web::scope("/users").service(router::users::create).service(
                    web::scope("")
                        .guard(AuthGuard)
                        .service(router::users::list_users),
                ),
            )
            .service(
                web::scope("/friends")
                    .guard(AuthGuard)
                    .service(router::friends::list_friends)
                    .service(router::friends::count_friends)
                    .service(router::friends::follow),
            )
            .service(
                web::scope("/messages")
                    .guard(AuthGuard)
                    .service(router::messages::list_messages)
                    .service(router::messages::send_message),
            )
    })
    .bind((config.ip_address, config.port))?
    .run()
    .await
}
