use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFriendsCountDto {
    pub count: usize,
}
