use chrono::{DateTime, Utc};
use domain::models::message::Message;
use domain::value_objects::{MessageId, UserId};
use errors::Error;
use std::future::Future;

pub trait MessagesRepository {
    fn next_identity(&self) -> impl Future<Output = MessageId> + Send;

    fn message_of_id(
        &self,
        id: MessageId,
    ) -> impl Future<Output = Result<Option<Message>, Error>> + Send;

    fn messages_from_interval(
        &self,
        from_id: UserId,
        to_id: UserId,
        from_date: Option<DateTime<Utc>>,
        to_date: Option<DateTime<Utc>>,
    ) -> impl Future<Output = Result<Vec<Message>, Error>> + Send;

    fn save(&self, message: Message) -> impl Future<Output = Result<(), Error>> + Send;
}
