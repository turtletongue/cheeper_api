use errors::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::models::message::Message;
use crate::value_objects::{Interest, MessageId, UserId};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "_id")]
    pub id: UserId,
    name: String,
    login: String,
    password_hash: String,
    following: HashSet<Interest>,
}

impl User {
    pub fn new(
        id: UserId,
        name: String,
        login: String,
        password_hash: String,
    ) -> Result<Self, Error> {
        if name.is_empty() {
            return Err(Error::InvalidStateError {
                message: "User name cannot be empty",
            });
        }

        if login.is_empty() {
            return Err(Error::InvalidStateError {
                message: "User login cannot be empty",
            });
        }

        if password_hash.is_empty() {
            return Err(Error::InvalidStateError {
                message: "Password hash cannot be empty",
            });
        }

        Ok(Self {
            id,
            name,
            login,
            password_hash,
            following: HashSet::new(),
        })
    }

    pub fn write_message_to(
        &self,
        other: &User,
        message_id: MessageId,
        text: String,
    ) -> Result<Message, Error> {
        Ok(Message::new(
            message_id,
            text,
            self.id.clone(),
            other.id.clone(),
        )?)
    }

    pub fn follow(&mut self, other: &User) -> Result<(), Error> {
        if self.id == other.id {
            return Err(Error::InvalidStateError {
                message: "You cannot follow yourself!",
            });
        }

        self.following.insert(Interest::new(other.id.clone()));

        Ok(())
    }

    pub fn unfollow(&mut self, other: &User) {
        self.following.remove(&other.id);
    }

    pub fn following_ids(&self) -> Vec<UserId> {
        self.following
            .iter()
            .map(|interest| interest.user_id.clone())
            .collect()
    }

    pub fn is_following(&self, other: &User) -> bool {
        self.following
            .iter()
            .any(|interest| interest.user_id == other.id)
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_password_hash(&self) -> String {
        self.password_hash.clone()
    }
}
