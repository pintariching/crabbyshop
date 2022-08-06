use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Json, Router};
use sqlx::PgPool;
use validator::Validate;

use crate::errors::ApiError;
use crate::models::discount::{Discount, DiscountInsert, DiscountUpdate};

pub fn get_routes() -> Router {
    Router::new()
        .route("/discount", get(fetch_all).post(create))
        .route("/discount/:id", get(fetch_one).patch(update).delete(delete))
        .route("/discount/:id/set-active", get(set_active))
        .route("/discount/:id/set-inactive", get(set_inactive))
}

async fn fetch_all(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    Discount::find_all(&pool)
        .await
        .map(|r| (StatusCode::OK, Json(r)))
        .map_err(|e| ApiError::internal_server_error(&e))
}

async fn fetch_one(Extension(pool): Extension<PgPool>, Path(id): Path<i64>) -> impl IntoResponse {
    Discount::find_by_id(id, &pool)
        .await
        .map(|r| (StatusCode::OK, Json(r)))
        .map_err(|e| ApiError::internal_server_error(&e))
}

async fn create(
    Extension(pool): Extension<PgPool>,
    Json(discount): Json<DiscountInsert>,
) -> impl IntoResponse {
    if let Err(e) = discount.validate() {
        return Err(ApiError::validation_error(e));
    }

    Discount::create(discount, &pool)
        .await
        .map(|r| (StatusCode::CREATED, Json(r)))
        .map_err(|e| ApiError::internal_server_error(&e))
}

async fn set_active(Extension(pool): Extension<PgPool>, Path(id): Path<i64>) -> impl IntoResponse {
    Discount::set_active(id, &pool)
        .await
        .map(|r| (StatusCode::OK, Json(r)))
        .map_err(|e| ApiError::internal_server_error(&e))
}

async fn set_inactive(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    Discount::set_inactive(id, &pool)
        .await
        .map(|r| (StatusCode::OK, Json(r)))
        .map_err(|e| ApiError::internal_server_error(&e))
}

async fn update(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i64>,
    Json(discount): Json<DiscountUpdate>,
) -> impl IntoResponse {
    if let Err(e) = discount.validate() {
        return Err(ApiError::validation_error(e));
    }

    Discount::update(id, discount, &pool)
        .await
        .map(|r| (StatusCode::ACCEPTED, Json(r)))
        .map_err(|e| ApiError::internal_server_error(&e))
}

async fn delete(Extension(pool): Extension<PgPool>, Path(id): Path<i64>) -> impl IntoResponse {
    Discount::delete(id, &pool)
        .await
        .map(|r| (StatusCode::NO_CONTENT, Json(r)))
        .map_err(|e| ApiError::internal_server_error(&e))
}
