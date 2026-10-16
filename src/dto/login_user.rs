use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginUserDto {
    pub email: String,
    pub password: String,
}
