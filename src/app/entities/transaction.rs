use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Transaction {
    pub id: Uuid,
    pub customer_id: Option<Uuid>,
    pub transaction_type: TransactionType,
    pub carat: i16,
    pub gold_type: GoldType,
    pub persian_date: String,
    pub timestamp: DateTime<Utc>,
    pub weight_gram: BigDecimal,
    pub total_price: i64,
    pub is_deleted: bool,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
#[sqlx(type_name = "transaction_type", rename_all = "lowercase")]
pub enum TransactionType {
    Sell,
    Buy,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::Type)]
#[sqlx(type_name = "gold_type", rename_all = "lowercase")]
pub enum GoldType {
    Fragile,
    Melted,
    Coin,
}
