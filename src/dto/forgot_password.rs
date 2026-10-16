use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ForgotPasswordDto {
    pub email: String,
}