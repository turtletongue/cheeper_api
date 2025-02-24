use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticateDto {
    pub login: String,
    pub password: String,
}
