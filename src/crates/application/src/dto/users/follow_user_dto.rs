use domain::value_objects::UserId;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FollowUserDto {
    pub user_id: UserId,
}
