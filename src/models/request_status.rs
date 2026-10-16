use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "varchar")]
pub enum RequestStatus {
    Pending,
    Approved,
    Completed,
    Cancelled,
}

impl fmt::Display for RequestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestStatus::Pending => write!(f, "Pending"),
            RequestStatus::Approved => write!(f, "Approved"),
            RequestStatus::Completed => write!(f, "Completed"),
            RequestStatus::Cancelled => write!(f, "Cancelled"),
        }
    }
}

impl FromStr for RequestStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Pending" => Ok(RequestStatus::Pending),
            "Approved" => Ok(RequestStatus::Approved),
            "Completed" => Ok(RequestStatus::Completed),
            "Cancelled" => Ok(RequestStatus::Cancelled),
            _ => Err(format!("Status inválido: {}", s)),
        }
    }
}