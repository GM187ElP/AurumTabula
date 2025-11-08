use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Customer {
    pub id: String, // UUID
    pub first_name: Option<String>,
    pub last_name: String,  // Required
    pub phone: Option<String>,
    pub national_code: Option<String>, // Melli Code (Optional)
    pub address: Option<String>,
    pub notes: Option<String>,
    pub created_at: Option<String>, // ISO time string from Supabase
    pub is_deleted: bool,  // Soft delete flag
}
