use serde::{Deserialize, Serialize};
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserDto {
    pub name: Option<String>,

    pub phone: Option<String>,

    pub city: Option<String>,

    pub state: Option<String>,
}
