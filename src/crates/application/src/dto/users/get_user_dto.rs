use domain::models::user::User;
use domain::value_objects::UserId;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserDto {
    pub id: UserId,
    pub name: String,
}

impl From<User> for GetUserDto {
    fn from(user: User) -> Self {
        Self {
            id: user.id.clone(),
            name: user.get_name(),
        }
    }
}
