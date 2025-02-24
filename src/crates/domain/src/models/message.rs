use chrono::{DateTime, Utc};
use errors::Error;
use serde::{Deserialize, Serialize};

use crate::value_objects::{MessageId, UserId};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    #[serde(rename = "_id")]
    pub id: MessageId,
    text: String,
    from_id: UserId,
    to_id: UserId,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    date: DateTime<Utc>,
}

impl Message {
    pub(crate) fn new(
        id: MessageId,
        text: String,
        from_id: UserId,
        to_id: UserId,
    ) -> Result<Self, Error> {
        if text.is_empty() {
            return Err(Error::InvalidStateError {
                message: "Message text cannot be empty",
            });
        }

        Ok(Self {
            id,
            text,
            from_id,
            to_id,
            date: Utc::now(),
        })
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn get_from_id(&self) -> &UserId {
        &self.from_id
    }

    pub fn get_to_id(&self) -> &UserId {
        &self.to_id
    }

    pub fn get_date(&self) -> &DateTime<Utc> {
        &self.date
    }
}
