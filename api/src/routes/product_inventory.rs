use rocket::http::Status;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use validator::Validate;

use crate::db::Db;
use crate::errors::ApiError;
use crate::models::product_inventory::{
	ProductInventory,
	ProductInventoryInsert,
	ProductInventoryUpdate
};

#[get("/inventory")]
pub async fn fetch_all(
	db: Connection<Db>
) -> Result<Json<Vec<ProductInventory>>, (Status, Json<ApiError>)> {
	ProductInventory::find_all(db)
		.await
		.map(Json)
		.map_err(|e| ApiError::internal_server_error(&e))
}

#[get("/inventory/<id>")]
pub async fn fetch_one(
	db: Connection<Db>,
	id: i64
) -> Result<Json<ProductInventory>, (Status, Json<ApiError>)> {
	ProductInventory::find_by_id(id, db)
		.await
		.map(Json)
		.map_err(|e| ApiError::internal_server_error(&e))
}


#[post("/inventory", format = "json", data = "<inventory>")]
pub async fn create(
	db: Connection<Db>,
	inventory: Json<ProductInventoryInsert>
) -> Result<Created<Json<ProductInventory>>, (Status, Json<ApiError>)> {
	if let Err(e) = inventory.validate() {
		return Err(ApiError::validation_error(e))
	}

	ProductInventory::create(inventory.into_inner(), db)
		.await
		.map(|response| Created::new(format!("/discount/{}", response.id)).body(Json(response)))
		.map_err(|e| ApiError::internal_server_error(&e))
}

#[patch("/inventory/<id>", format = "json", data = "<inventory>")]
pub async fn update(
	db: Connection<Db>,
	id: i64,
	inventory: Json<ProductInventoryUpdate>
) -> Result<Json<ProductInventory>, (Status, Json<ApiError>)> {
	if let Err(e) = inventory.validate() {
		return Err(ApiError::validation_error(e))
	}

	ProductInventory::update(id, inventory.into_inner(), db)
		.await
		.map(Json)
		.map_err(|e| ApiError::internal_server_error(&e))
}

#[delete("/inventory/<id>")]
pub async fn delete(
	db: Connection<Db>,
	id: i64
) -> Result<Json<u64>, (Status, Json<ApiError>)> {
	ProductInventory::delete(id, db)
		.await
		.map(Json)
		.map_err(|e| ApiError::internal_server_error(&e))
}