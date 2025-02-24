use domain::value_objects::UserId;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFriendCandidateDto {
    pub id: UserId,
    pub name: String,
    pub is_friend: bool,
}
