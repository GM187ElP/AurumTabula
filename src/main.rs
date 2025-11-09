use axum::{Router, response::Html, routing::get};
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;
use tokio::net::TcpListener;

use aurumtabula::app::routes::transactions;
use aurumtabula::app::state::AppState;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db = PgPool::connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Failed to connect to database");

    let app_state = AppState { db };

    let app = Router::new()
        .route("/", get(homepage))
        .merge(transactions::routes())
        .with_state(app_state);

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind port");

    println!("Server running at http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn homepage() -> Html<&'static str> {
    Html(
        r#"
        <h1>Welcome to AurumTabula</h1>
        <p><a href="/transactions/new">➕ Add New Transaction</a></p>
    "#,
    )
}
