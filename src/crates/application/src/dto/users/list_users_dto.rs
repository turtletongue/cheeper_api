use serde::Serialize;

use super::GetFriendCandidateDto;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListUsersDto {
    pub items: Vec<GetFriendCandidateDto>,
}
