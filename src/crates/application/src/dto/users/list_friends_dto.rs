use serde::Serialize;

use super::GetUserDto;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListFriendsDto {
    pub items: Vec<GetUserDto>,
}
