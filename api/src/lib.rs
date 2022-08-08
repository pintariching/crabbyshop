use axum::{Extension, Router};
use dotenv::dotenv;
use routes::{category, discount};
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

//mod auth;
mod errors;
mod models;
mod routes;

pub async fn create_app() -> Router {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("LOG_LEVEL")
                .unwrap_or_else(|_| "crabbyshop=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_connection_string = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .idle_timeout(Duration::from_secs(5))
        .connect(&db_connection_string)
        .await
        .expect("Can't connect to database");

    let routes = Router::new()
        .merge(discount::get_routes())
        .merge(category::get_routes());

    Router::new()
        .nest("/api/v1", routes)
        .layer(Extension(pool))
        .layer(TraceLayer::new_for_http())
}
