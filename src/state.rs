use infrastructure::hashing::Argon2PasswordHasher;
use infrastructure::repositories::{MongoMessagesRepository, MongoUsersRepository};

pub struct AppState {
    pub users_repository: MongoUsersRepository,
    pub messages_repository: MongoMessagesRepository,
    pub password_hasher: Argon2PasswordHasher,
}
