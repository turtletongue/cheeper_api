use chrono::{DateTime, Utc};
use domain::models::message::Message;
use domain::value_objects::{MessageId, UserId};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMessageDto {
    pub id: MessageId,
    pub text: String,
    pub from_id: UserId,
    pub to_id: UserId,
    pub date: DateTime<Utc>,
}

impl From<Message> for GetMessageDto {
    fn from(message: Message) -> Self {
        Self {
            id: message.id.clone(),
            text: message.get_text().to_string(),
            to_id: message.get_to_id().clone(),
            from_id: message.get_from_id().clone(),
            date: message.get_date().clone(),
        }
    }
}
