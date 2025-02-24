use domain::value_objects::UserId;
use errors::Error;

use crate::dto::authentication::AuthenticateDto;
use crate::hashing::PasswordHasher;
use crate::repositories::UsersRepository;

pub struct AuthenticationService<'a, K: UsersRepository, V: PasswordHasher> {
    users_repository: &'a K,
    password_hasher: &'a V,
}

impl<'a, K: UsersRepository, V: PasswordHasher> AuthenticationService<'a, K, V> {
    pub fn new(users_repository: &'a K, password_hasher: &'a V) -> Self {
        Self {
            users_repository,
            password_hasher,
        }
    }

    pub async fn authenticate(&self, dto: AuthenticateDto) -> Result<UserId, Error> {
        let user = self.users_repository.user_of_login(dto.login).await?;

        if let None = user {
            return Err(Error::UnauthorizedError {});
        }

        let user = user.unwrap();

        match self
            .password_hasher
            .verify(dto.password, user.get_password_hash())
        {
            Ok(is_ok) if is_ok => Ok(user.id.clone()),
            Ok(_) => Err(Error::UnauthorizedError {}),
            Err(_) => Err(Error::UnauthorizedError {}),
        }
    }
}
