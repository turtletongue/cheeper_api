use domain::models::user::User;
use domain::value_objects::UserId;
use errors::Error;

use crate::dto::users::{
    CreateUserDto, FollowUserDto, GetFriendCandidateDto, GetFriendsCountDto, ListFriendsDto,
    ListUsersDto,
};
use crate::hashing::PasswordHasher;
use crate::repositories::UsersRepository;

pub struct UsersService<'a, K: UsersRepository, V: PasswordHasher> {
    users_repository: &'a K,
    password_hasher: &'a V,
}

impl<'a, K: UsersRepository, V: PasswordHasher> UsersService<'a, K, V> {
    pub fn new(users_repository: &'a K, password_hasher: &'a V) -> Self {
        Self {
            users_repository,
            password_hasher,
        }
    }

    pub async fn list_users(&self, user_id: UserId) -> Result<ListUsersDto, Error> {
        let user = self.users_repository.user_of_id(user_id).await?;

        if let None = user {
            return Err(Error::NotFoundError {
                message: "Executor not found",
            });
        }

        let user = user.unwrap();

        let users = self.users_repository.all_users(user.id.clone()).await?;

        Ok(ListUsersDto {
            items: users
                .into_iter()
                .map(|candidate| GetFriendCandidateDto {
                    id: candidate.id.clone(),
                    name: candidate.get_name().clone(),
                    is_friend: candidate.is_following(&user) && user.is_following(&candidate),
                })
                .collect(),
        })
    }

    pub async fn list_friends(&self, user_id: UserId) -> Result<ListFriendsDto, Error> {
        let user = self.users_repository.user_of_id(user_id).await?;

        if let None = user {
            return Err(Error::NotFoundError {
                message: "Follower not found",
            });
        }

        let user = user.unwrap();

        let friends = self.users_repository.friends(&user).await?;

        Ok(ListFriendsDto {
            items: friends.into_iter().map(|friend| friend.into()).collect(),
        })
    }

    pub async fn get_friends_count(&self, user_id: UserId) -> Result<GetFriendsCountDto, Error> {
        let user = self.users_repository.user_of_id(user_id).await?;

        if let None = user {
            return Err(Error::NotFoundError {
                message: "Follower not found",
            });
        }

        let user = user.unwrap();

        let count = self.users_repository.friends_count(&user).await?;

        Ok(GetFriendsCountDto { count })
    }

    pub async fn add_user(&self, dto: CreateUserDto) -> Result<(), Error> {
        let user_id = self.users_repository.next_identity().await;
        let password_hash = self.password_hasher.hash(dto.password)?;
        let user = User::new(user_id, dto.name, dto.login, password_hash)?;

        self.users_repository.save(user).await?;

        Ok(())
    }

    pub async fn follow(&self, dto: FollowUserDto, follower_id: UserId) -> Result<(), Error> {
        let follower = self.users_repository.user_of_id(follower_id).await?;

        if let None = follower {
            return Err(Error::NotFoundError {
                message: "Follower not found",
            });
        }

        let user = self.users_repository.user_of_id(dto.user_id).await?;

        if let None = user {
            return Err(Error::NotFoundError {
                message: "User not found",
            });
        }

        let user = user.unwrap();
        let mut follower = follower.unwrap();

        follower.follow(&user)?;
        self.users_repository.save(follower).await?;

        Ok(())
    }
}
