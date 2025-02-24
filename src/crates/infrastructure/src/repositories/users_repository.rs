use application::repositories::UsersRepository;
use domain::models::user::User;
use domain::value_objects::UserId;
use errors::Error;
use futures::TryStreamExt;
use log::error;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::{Client, Collection};
use std::sync::Arc;

use crate::database::DATABASE_NAME;

const USER_COLLECTION: &'static str = "users";

#[derive(Clone)]
pub struct MongoUsersRepository {
    client: Arc<Client>,
}

impl MongoUsersRepository {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }
}

impl UsersRepository for MongoUsersRepository {
    async fn next_identity(&self) -> UserId {
        UserId(ObjectId::new().to_string())
    }

    async fn user_of_id(&self, id: UserId) -> Result<Option<User>, Error> {
        let collection: Collection<User> = self
            .client
            .database(DATABASE_NAME)
            .collection(USER_COLLECTION);

        match collection.find_one(doc! { "_id": id.0 }).await {
            Ok(result) => Ok(result),
            Err(error) => {
                error!("Retrieving user by ID failed: {}", error);
                Err(Error::DatabaseError {})
            }
        }
    }

    async fn user_of_login(&self, login: String) -> Result<Option<User>, Error> {
        let collection: Collection<User> = self
            .client
            .database(DATABASE_NAME)
            .collection(USER_COLLECTION);

        match collection.find_one(doc! { "login": login }).await {
            Ok(result) => Ok(result),
            Err(error) => {
                error!("Retrieving user by login failed: {}", error);
                Err(Error::DatabaseError {})
            }
        }
    }

    async fn all_users(&self, user_id: UserId) -> Result<Vec<User>, Error> {
        let collection: Collection<User> = self
            .client
            .database(DATABASE_NAME)
            .collection(USER_COLLECTION);

        match collection
            .find(doc! {
                "_id": doc! {
                    "$ne": user_id.0,
                }
            })
            .sort(doc! { "name": 1 })
            .await
        {
            Ok(cursor) => Ok(cursor.try_collect().await.unwrap_or_else(|_| vec![])),
            Err(error) => {
                error!("Retrieving all users failed: {}", error);
                Err(Error::DatabaseError {})
            }
        }
    }

    async fn friends(&self, follower: &User) -> Result<Vec<User>, Error> {
        let collection: Collection<User> = self
            .client
            .database(DATABASE_NAME)
            .collection(USER_COLLECTION);

        match collection
            .find(doc! {
                "_id": doc! {
                    "$in": follower.following_ids()
                        .into_iter()
                        .map(|id| id.0)
                        .collect::<Vec<String>>(),
                },
                "following": doc! {
                    "$elemMatch": doc! {
                        "userId": follower.id.clone().0,
                    }
                }
            })
            .sort(doc! { "name": 1 })
            .await
        {
            Ok(cursor) => Ok(cursor.try_collect().await.unwrap_or_else(|_| vec![])),
            Err(error) => {
                error!("Retrieving users by follower ID failed: {}", error);
                Err(Error::DatabaseError {})
            }
        }
    }

    async fn friends_count(&self, follower: &User) -> Result<usize, Error> {
        let collection: Collection<User> = self
            .client
            .database(DATABASE_NAME)
            .collection(USER_COLLECTION);

        match collection
            .count_documents(doc! {
                "_id": doc! {
                    "$in": follower.following_ids()
                        .into_iter()
                        .map(|id| id.0)
                        .collect::<Vec<String>>(),
                },
                "following": doc! {
                    "$elemMatch": doc! {
                        "userId": follower.id.clone().0,
                    }
                }
            })
            .await
        {
            Ok(count) => Ok(count as usize),
            Err(error) => {
                error!("Counting friends by user ID failed: {}", error);
                Err(Error::DatabaseError {})
            }
        }
    }

    async fn save(&self, user: User) -> Result<(), Error> {
        let collection: Collection<User> = self
            .client
            .database(DATABASE_NAME)
            .collection(USER_COLLECTION);

        match collection
            .find_one_and_replace(doc! { "_id": user.id.clone().0 }, user)
            .upsert(true)
            .await
        {
            Ok(_) => Ok(()),
            Err(error) => {
                error!("Saving user failed: {}", error);
                Err(Error::DatabaseError {})
            }
        }
    }
}
