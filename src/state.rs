use application::services::{AuthenticationService, MessageService, UsersService};
use infrastructure::hashing::Argon2PasswordHasher;
use infrastructure::repositories::{MongoMessagesRepository, MongoUsersRepository};
use mongodb::Client;
use std::sync::Arc;

pub struct AppState {
    pub authentication_service: AuthenticationService<MongoUsersRepository, Argon2PasswordHasher>,
    pub users_service: UsersService<MongoUsersRepository, Argon2PasswordHasher>,
    pub message_service: MessageService<MongoMessagesRepository, MongoUsersRepository>,
}

impl AppState {
    pub fn new(mongo_client: Arc<Client>) -> Self {
        let users_repository = MongoUsersRepository::new(mongo_client.clone());
        let messages_repository = MongoMessagesRepository::new(mongo_client.clone());
        let password_hasher = Argon2PasswordHasher::new();

        Self {
            authentication_service: AuthenticationService::new(
                users_repository.clone(),
                password_hasher.clone(),
            ),
            users_service: UsersService::new(users_repository.clone(), password_hasher.clone()),
            message_service: MessageService::new(messages_repository, users_repository),
        }
    }
}
