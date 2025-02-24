use domain::models::user::User;
use domain::value_objects::UserId;
use errors::Error;
use std::future::Future;

pub trait UsersRepository {
    fn next_identity(&self) -> impl Future<Output = UserId> + Send;

    fn user_of_id(&self, id: UserId) -> impl Future<Output = Result<Option<User>, Error>> + Send;

    fn user_of_login(
        &self,
        login: String,
    ) -> impl Future<Output = Result<Option<User>, Error>> + Send;

    fn all_users(&self, user_id: UserId) -> impl Future<Output = Result<Vec<User>, Error>> + Send;

    fn friends(&self, follower: &User) -> impl Future<Output = Result<Vec<User>, Error>> + Send;

    fn friends_count(&self, follower: &User) -> impl Future<Output = Result<usize, Error>> + Send;

    fn save(&self, user: User) -> impl Future<Output = Result<(), Error>> + Send;
}
