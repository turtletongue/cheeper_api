use application::hashing;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use errors::Error;
use log::error;

pub struct Argon2PasswordHasher;

impl Argon2PasswordHasher {
    pub fn new() -> Self {
        Self {}
    }
}

impl hashing::PasswordHasher for Argon2PasswordHasher {
    fn hash(&self, password: String) -> Result<String, Error> {
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        match argon2.hash_password(password.into_bytes().as_slice(), &salt) {
            Ok(result) => Ok(result.to_string()),
            Err(error) => {
                error!("Failed to hash password: {}", error);
                Err(Error::PasswordError {})
            }
        }
    }

    fn verify(&self, password: String, password_hash: String) -> Result<bool, Error> {
        let parsed_hash = PasswordHash::new(&password_hash);

        if let Err(error) = parsed_hash {
            error!("Failed to parse password hash: {}", error);

            return Err(Error::PasswordError {});
        }

        let parsed_hash = parsed_hash.unwrap();

        Ok(Argon2::default()
            .verify_password(password.into_bytes().as_slice(), &parsed_hash)
            .is_ok())
    }
}
