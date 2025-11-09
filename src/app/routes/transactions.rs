use crate::app::models::customer_select::CustomerSelect;
use crate::app::services::customer_service;
use crate::app::state::AppState;

use askama::Template;
use axum::{Router, extract::State, response::Html, routing::get};

#[derive(Template)]
#[template(path = "add_transaction.html")]
pub struct AddTransactionTemplate {
    pub customers: Vec<CustomerSelect>,
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/transactions/new", get(show_form))
}

async fn show_form(State(state): State<AppState>) -> Html<String> {
    let customers = customer_service::fetch_all_customers(&state.db)
        .await
        .unwrap();

    let template = AddTransactionTemplate { customers };

    Html(template.render().unwrap())
}
