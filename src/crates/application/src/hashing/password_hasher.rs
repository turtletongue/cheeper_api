use errors::Error;

pub trait PasswordHasher {
    fn hash(&self, password: String) -> Result<String, Error>;

    fn verify(&self, password: String, password_hash: String) -> Result<bool, Error>;
}
