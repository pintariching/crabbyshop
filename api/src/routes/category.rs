use rocket::http::Status;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use validator::Validate;

use crate::db::Db;
use crate::errors::ApiError;
use crate::models::category::{
	Category,
	CategoryInsert,
	CategoryUpdate, 
	CategorySorted
};

#[get("/category")]
pub async fn fetch_all(
	db: Connection<Db>
) -> Result<Json<Vec<CategorySorted>>, (Status, Json<ApiError>)> {
	let categories = match Category::find_all(db)
		.await
		.map_err(|e| ApiError::internal_server_error(&e)) {
		Ok(c) => c,
		Err(e) => return Err(e),
	};

	match CategorySorted::from_categories(categories) {
		Some(c) => Ok(Json(c)),
		None => Err(ApiError::internal_server_error("unable to sort categories")),
	}
}

#[get("/category/<id>")]
pub async fn fetch_one(
	db: Connection<Db>,
	id: i64
) -> Result<Json<Vec<CategorySorted>>, (Status, Json<ApiError>)> {
	let categories = match Category::find_by_id(id, db)
		.await
		.map_err(|e| ApiError::internal_server_error(&e)) {
		Ok(c) => if c.len() > 0 {
			c
		} else {
			return Err(ApiError::not_found(format!("value with id: {} not found", id).as_str()))
		},
		Err(e) => return Err(e),
	};

	match CategorySorted::from_categories(categories) {
		Some(c) => Ok(Json(c)),
		None => Err(ApiError::internal_server_error("unable to sort categories")),
	}
}

#[post("/category", format = "json", data = "<category>")]
pub async fn create(
	db: Connection<Db>,
	category: Json<CategoryInsert>
) -> Result<Created<Json<Category>>, (Status, Json<ApiError>)> {
	if let Err(e) = category.validate() {
		return Err(ApiError::validation_error(e))
	}

	Category::create(category.into_inner(), db)
		.await
		.map(|response| Created::new(format!("/category/{}", response.id)).body(Json(response)))
		.map_err(|e| ApiError::internal_server_error(&e))
}

#[patch("/category/<id>", format = "json", data = "<category>")]
pub async fn update(
	db: Connection<Db>,
	id: i64,
	category: Json<CategoryUpdate>
) -> Result<Json<Category>, (Status, Json<ApiError>)> {
	if let Err(e) = category.validate() {
		return Err(ApiError::validation_error(e))
	}

	Category::update(id, category.into_inner(), db)
		.await
		.map(Json)
		.map_err(|e| ApiError::internal_server_error(&e))
}

#[delete("/category/<id>")]
pub async fn delete(
	db: Connection<Db>,
	id: i64
) -> Result<Json<u64>, (Status, Json<ApiError>)> {
	Category::delete(id, db)
		.await
		.map(Json)
		.map_err(|e| ApiError::internal_server_error(&e))
}