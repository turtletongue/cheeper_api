use chrono::{DateTime, Utc};
use domain::value_objects::UserId;
use serde::{Deserialize, Serialize};

use super::GetMessageDto;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListMessagesFromIntervalParams {
    pub sender_id: UserId,
    pub from_date: Option<DateTime<Utc>>,
    pub to_date: Option<DateTime<Utc>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListMessagesFromIntervalDto {
    pub items: Vec<GetMessageDto>,
}
