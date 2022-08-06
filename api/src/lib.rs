use axum::{Extension, Router};
use dotenv::dotenv;
use routes::discount;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

//mod auth;
mod errors;
mod models;
mod routes;

pub async fn create_app() -> Router {
    dotenv().ok();
    let db_connection_string = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .idle_timeout(Duration::from_secs(5))
        .connect(&db_connection_string)
        .await
        .expect("can connect to database");

    let routes = Router::new().nest("/", discount::get_routes());

    Router::new().nest("/api/v1", routes).layer(Extension(pool))
}
