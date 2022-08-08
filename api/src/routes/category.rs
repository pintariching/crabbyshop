use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Json, Router};
use sqlx::PgPool;
use validator::Validate;

use crate::errors::ApiError;
use crate::models::category::{Category, CategoryInsert, CategoryUpdate};

pub fn get_routes() -> Router {
    Router::new()
        .route("/category", get(fetch_all).post(create))
        .route("/category/:id", get(fetch_one).patch(update).delete(delete))
}

async fn fetch_all(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    Category::find_all(&pool)
        .await
        .map(|r| (StatusCode::OK, Json(r)))
        .map_err(|e| ApiError::internal_server_error(&e))
}

async fn fetch_one(Extension(pool): Extension<PgPool>, Path(id): Path<i64>) -> impl IntoResponse {
    Category::find_by_id(id, &pool)
        .await
        .map(|r| (StatusCode::OK, Json(r)))
        .map_err(|e| ApiError::internal_server_error(&e))
}

async fn create(
    Extension(pool): Extension<PgPool>,
    Json(category): Json<CategoryInsert>,
) -> impl IntoResponse {
    if let Err(e) = category.validate() {
        return Err(ApiError::validation_error(e));
    }

    Category::create(category, &pool)
        .await
        .map(|r| (StatusCode::CREATED, Json(r)))
        .map_err(|e| ApiError::internal_server_error(&e))
}

async fn update(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i64>,
    Json(category): Json<CategoryUpdate>,
) -> impl IntoResponse {
    if let Err(e) = category.validate() {
        return Err(ApiError::validation_error(e));
    }

    Category::update(id, category, &pool)
        .await
        .map(|r| (StatusCode::ACCEPTED, Json(r)))
        .map_err(|e| ApiError::internal_server_error(&e))
}

async fn delete(Extension(pool): Extension<PgPool>, Path(id): Path<i64>) -> impl IntoResponse {
    Category::delete(id, &pool)
        .await
        .map(|r| (StatusCode::NO_CONTENT, Json(r)))
        .map_err(|e| ApiError::internal_server_error(&e))
}
