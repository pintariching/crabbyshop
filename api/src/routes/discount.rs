use rocket::http::Status;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use validator::Validate;

use crate::db::Db;
use crate::errors::ApiError;
use crate::models::authentication::JwtToken;
use crate::models::discount::{
	Discount,
	DiscountInsert,
	DiscountUpdate
};

#[get("/discount")]
pub async fn fetch_all(
	db: Connection<Db>,
	token: Result<JwtToken, String>
) -> Result<Json<Vec<Discount>>, (Status, Json<ApiError>)> {
	Discount::find_all(db)
		.await
		.map(Json)
		.map_err(|e| ApiError::internal_server_error(&e))
}

#[get("/discount/<id>")]
pub async fn fetch_one(
	db: Connection<Db>,
	id: i64
) -> Result<Json<Discount>, (Status, Json<ApiError>)> {
	Discount::find_by_id(id, db)
		.await
		.map(Json)
		.map_err(|e| ApiError::internal_server_error(&e))
}


#[post("/discount", format = "json", data = "<discount>")]
pub async fn create(
	db: Connection<Db>,
	discount: Json<DiscountInsert>,
	token: Result<JwtToken, String>
) -> Result<Created<Json<Discount>>, (Status, Json<ApiError>)> {
	if let Err(e) = discount.validate() {
		return Err(ApiError::validation_error(e))
	}

	match token {
		Ok(t) => if !t.is_admin() { return Err(ApiError::bad_request("user is not an admin"))},
    	Err(e) => return Err(ApiError::bad_request(&e)),
	}

	Discount::create(discount.into_inner(), db)
		.await
		.map(|response| Created::new(format!("/discount/{}", response.id)).body(Json(response)))
		.map_err(|e| ApiError::internal_server_error(&e))
}

#[get("/discount/<id>/set-active")]
pub async fn set_active(
	db: Connection<Db>,
	id: i64
) -> Result<Json<u64>, (Status, Json<ApiError>)> {
	Discount::set_active(id, db)
		.await
		.map(Json)
		.map_err(|e| ApiError::internal_server_error(&e))
}

#[get("/discount/<id>/set-inactive")]
pub async fn set_inactive(
	db: Connection<Db>,
	id: i64
) -> Result<Json<u64>, (Status, Json<ApiError>)> {
	Discount::set_inactive(id, db)
		.await
		.map(Json)
		.map_err(|e| ApiError::internal_server_error(&e))
}

#[patch("/discount/<id>", format = "json", data = "<discount>")]
pub async fn update(
	db: Connection<Db>,
	id: i64,
	discount: Json<DiscountUpdate>
) -> Result<Json<Discount>, (Status, Json<ApiError>)> {
	if let Err(e) = discount.validate() {
		return Err(ApiError::validation_error(e))
	}

	Discount::update(id, discount.into_inner(), db)
		.await
		.map(Json)
		.map_err(|e| ApiError::internal_server_error(&e))
}

#[delete("/discount/<id>")]
pub async fn delete(
	db: Connection<Db>,
	id: i64
) -> Result<Json<u64>, (Status, Json<ApiError>)> {
	Discount::delete(id, db)
		.await
		.map(Json)
		.map_err(|e| ApiError::internal_server_error(&e))
}