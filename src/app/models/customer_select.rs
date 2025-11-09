use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct CustomerSelect {
    pub id: Uuid,
    pub first_name: String,
}
