use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ResetPasswordDto {
    pub token: String,
    pub password: String,
    pub password_confirmation: String,
}