use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Json, Router};
use sqlx::PgPool;
use validator::Validate;

use crate::errors::ApiError;
use crate::models::product::{Product, ProductInsert, ProductUpdate};
use crate::models::product_inventory::{ProductInventory, ProductInventoryInsert};

use super::Params;

pub fn get_routes() -> Router {
    Router::new()
        .route("/product", get(fetch_all).post(create))
        .route("/product/query", get(fetch_by_category))
        .route("/product/:id", get(fetch_one).patch(update).delete(delete))
}

async fn fetch_all(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    Product::find_all(&pool)
        .await
        .map(Json)
        .map_err(|e| ApiError::internal_server_error(&e))
}

async fn fetch_one(Extension(pool): Extension<PgPool>, Path(id): Path<i64>) -> impl IntoResponse {
    Product::find_by_id(id, &pool)
        .await
        .map(Json)
        .map_err(|e| ApiError::internal_server_error(&e))
}

pub async fn fetch_by_category(
    Extension(pool): Extension<PgPool>,
    Query(params): Query<Params>,
) -> impl IntoResponse {
    Product::find_by_category(params.category_id, &pool)
        .await
        .map(Json)
        .map_err(|e| ApiError::internal_server_error(&e))
}

async fn create(
    Extension(pool): Extension<PgPool>,
    Json(mut product): Json<ProductInsert>,
) -> impl IntoResponse {
    if let Err(e) = product.validate() {
        return Err(ApiError::validation_error(e));
    }

    let inventory = match ProductInventory::create(ProductInventoryInsert::new(0), &pool)
        .await
        .map_err(|e| ApiError::internal_server_error(&e))
    {
        Ok(i) => i,
        Err(e) => return Err(e),
    };

    product.inventory_id = Some(inventory.id);

    Product::create(product, &pool)
        .await
        .map(|r| (StatusCode::CREATED, Json(r)))
        .map_err(|e| ApiError::internal_server_error(&e))
}

async fn update(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i64>,
    Json(product): Json<ProductUpdate>,
) -> impl IntoResponse {
    if let Err(e) = product.validate() {
        return Err(ApiError::validation_error(e));
    }

    Product::update(id, product, &pool)
        .await
        .map(Json)
        .map_err(|e| ApiError::internal_server_error(&e))
}

async fn delete(Extension(pool): Extension<PgPool>, Path(id): Path<i64>) -> impl IntoResponse {
    Product::delete(id, &pool)
        .await
        .map(Json)
        .map_err(|e| ApiError::internal_server_error(&e))
}
