use axum::{Router, response::Html, routing::get};
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;
use tokio::net::TcpListener;

use aurumtabula::app::routes::transactions;
use aurumtabula::app::state::AppState;

async fn homepage() -> Html<&'static str> {
    Html(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>AurumTabula</title>
            <meta charset="UTF-8">
        </head>
        <body>
            <h1>Welcome to AurumTabula</h1>
            <p><a href="/transactions/new">➕ Add New Transaction</a></p>
        </body>
        </html>
    "#,
    )
}

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

    // Bind to 0.0.0.0 to accept external connections
    // Use PORT environment variable from Fly.io, default to 3000
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let listener = TcpListener::bind(&addr).await.expect("Failed to bind port");

    println!("Server running on {}", addr);
    axum::serve(listener, app).await.unwrap();
}
