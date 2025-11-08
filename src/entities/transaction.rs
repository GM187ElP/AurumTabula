use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub id: String, // UUID
    pub customer_id: Option<String>, // Foreign key (nullable)
    pub transaction_type: TransactionType, // buy/sell
    pub carat: u16,  // e.g. 750, 916, 999 (max 1000)
    pub gold_type: GoldType, // coin, bar, melted gold, scrap
    pub persian_date: String, // e.g. "1403-08-20" (Jalali or Gregorian)
    pub timestamp: DateTime<Utc>,  // exact moment in time (PostgreSQL TIMESTAMPTZ)
    pub weight_gram: f64, // up to 3 decimals (e.g. 12.345)
    pub total_price: i64, // (rial or toman)
    pub is_deleted: bool,

    pub notes: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")] // Ensures JSON is like "buy", "sell"
pub enum TransactionType {
    Sell,
    Buy,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")] // JSON: "melted", "fragile", "coin"
pub enum GoldType {
    Fragile,
    Melted,
    Coin,
}

