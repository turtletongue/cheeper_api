use crate::database::DATABASE_NAME;
use application::repositories::MessagesRepository;
use chrono::{DateTime, Utc};
use domain::models::message::Message;
use domain::value_objects::{MessageId, UserId};
use errors::Error;
use futures::TryStreamExt;
use log::error;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::{Client, Collection};
use std::sync::Arc;

const MESSAGE_COLLECTION: &'static str = "messages";

pub struct MongoMessagesRepository {
    client: Arc<Client>,
}

impl MongoMessagesRepository {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }
}

impl MessagesRepository for MongoMessagesRepository {
    async fn next_identity(&self) -> MessageId {
        MessageId(ObjectId::new().to_string())
    }

    async fn message_of_id(&self, id: MessageId) -> Result<Option<Message>, Error> {
        let collection: Collection<Message> = self
            .client
            .database(DATABASE_NAME)
            .collection(MESSAGE_COLLECTION);

        match collection.find_one(doc! { "_id": id.0 }).await {
            Ok(result) => Ok(result),
            Err(error) => {
                error!("Retrieving message by ID failed: {}", error);
                Err(Error::DatabaseError {})
            }
        }
    }

    async fn messages_from_interval(
        &self,
        from_id: UserId,
        to_id: UserId,
        from_date: Option<DateTime<Utc>>,
        to_date: Option<DateTime<Utc>>,
    ) -> Result<Vec<Message>, Error> {
        let collection: Collection<Message> = self
            .client
            .database(DATABASE_NAME)
            .collection(MESSAGE_COLLECTION);

        let mut filter = bson::Document::from(doc! {
            "$or": vec![
                doc! { "fromId": &from_id.0, "toId": &to_id.0 },
                doc! { "fromId": &to_id.0, "toId": &from_id.0 },
            ],
        });

        if from_date.is_some() {
            filter
                .entry(String::from("date"))
                .or_insert(bson::Document::new().into())
                .as_document_mut()
                .unwrap()
                .insert("$gte", bson::DateTime::from_chrono(from_date.unwrap()));
        }

        if to_date.is_some() {
            filter
                .entry(String::from("date"))
                .or_insert(bson::Document::new().into())
                .as_document_mut()
                .unwrap()
                .insert("$lte", bson::DateTime::from_chrono(to_date.unwrap()));
        }

        match collection.find(filter).await {
            Ok(cursor) => Ok(cursor.try_collect().await.unwrap_or_else(|_| vec![])),
            Err(error) => {
                error!("Retrieving messages from date interval failed: {}", error);
                Err(Error::DatabaseError {})
            }
        }
    }

    async fn save(&self, message: Message) -> Result<(), Error> {
        let collection: Collection<Message> = self
            .client
            .database(DATABASE_NAME)
            .collection(MESSAGE_COLLECTION);

        match collection
            .find_one_and_replace(doc! { "_id": message.id.clone().0 }, message)
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
