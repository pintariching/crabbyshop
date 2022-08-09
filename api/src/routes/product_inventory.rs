use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Json, Router};
use sqlx::PgPool;
use validator::Validate;

use crate::errors::ApiError;
use crate::models::product_inventory::{
    ProductInventory, ProductInventoryInsert, ProductInventoryUpdate,
};

pub fn get_routes() -> Router {
    Router::new()
        .route("/inventory", get(fetch_all).post(create))
        .route(
            "/inventory/:id",
            get(fetch_one).patch(update).delete(delete),
        )
}

async fn fetch_all(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    ProductInventory::find_all(&pool)
        .await
        .map(Json)
        .map_err(|e| ApiError::internal_server_error(&e))
}

async fn fetch_one(Extension(pool): Extension<PgPool>, Path(id): Path<i64>) -> impl IntoResponse {
    ProductInventory::find_by_id(id, &pool)
        .await
        .map(Json)
        .map_err(|e| ApiError::internal_server_error(&e))
}

async fn create(
    Extension(pool): Extension<PgPool>,
    Json(inventory): Json<ProductInventoryInsert>,
) -> impl IntoResponse {
    if let Err(e) = inventory.validate() {
        return Err(ApiError::validation_error(e));
    }

    ProductInventory::create(inventory, &pool)
        .await
        .map(|r| (StatusCode::CREATED, Json(r)))
        .map_err(|e| ApiError::internal_server_error(&e))
}

async fn update(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i64>,
    Json(inventory): Json<ProductInventoryUpdate>,
) -> impl IntoResponse {
    if let Err(e) = inventory.validate() {
        return Err(ApiError::validation_error(e));
    }

    ProductInventory::update(id, inventory, &pool)
        .await
        .map(Json)
        .map_err(|e| ApiError::internal_server_error(&e))
}

async fn delete(Extension(pool): Extension<PgPool>, Path(id): Path<i64>) -> impl IntoResponse {
    ProductInventory::delete(id, &pool)
        .await
        .map(Json)
        .map_err(|e| ApiError::internal_server_error(&e))
}
