use crate::app::models::customer_select::CustomerSelect;
use sqlx::{Pool, Postgres};

pub async fn fetch_all_customers(db: &Pool<Postgres>) -> Result<Vec<CustomerSelect>, sqlx::Error> {
    let customers = sqlx::query_as::<_, CustomerSelect>(
        "SELECT id, first_name FROM customers WHERE is_deleted = FALSE",
    )
    .fetch_all(db)
    .await?;

    Ok(customers)
}
