use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Customer {
    pub id: Uuid,
    pub first_name: Option<String>,
    pub last_name: String,
    pub phone: Option<String>,
    pub national_code: Option<String>,
    pub address: Option<String>,
    pub notes: Option<String>,
    pub is_deleted: bool,
    pub created_at: NaiveDate,
}
