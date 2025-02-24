use domain::value_objects::UserId;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageDto {
    pub text: String,
    pub receiver_id: UserId,
}
