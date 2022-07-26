use rocket::http::Status;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use validator::Validate;

use crate::db::Db;
use crate::errors::ApiError;
use crate::models::product::{
	Product,
	ProductInsert,
	ProductUpdate
};
use crate::models::product_inventory::{ProductInventory, ProductInventoryInsert};

#[get("/product")]
pub async fn fetch_all(
	db: Connection<Db>
) -> Result<Json<Vec<Product>>, (Status, Json<ApiError>)> {
	Product::find_all(db)
		.await
		.map(Json)
		.map_err(|e| ApiError::internal_server_error(&e))
}

#[get("/product/<id>")]
pub async fn fetch_one(
	db: Connection<Db>,
	id: i64
) -> Result<Json<Product>, (Status, Json<ApiError>)> {
	Product::find_by_id(id, db)
		.await
		.map(Json)
		.map_err(|e| ApiError::internal_server_error(&e))
}

#[get("/product?<category_id>")]
pub async fn fetch_by_category(
	db: Connection<Db>,
	category_id: i64
) -> Result<Json<Vec<Product>>, (Status, Json<ApiError>)> {
	Product::find_by_category(category_id, db)
		.await
		.map(Json)
		.map_err(|e| ApiError::internal_server_error(&e))
}

#[post("/product", format = "json", data = "<product>")]
pub async fn create(
	db: Connection<Db>,
	db_inventory: Connection<Db>,
	mut product: Json<ProductInsert>
) -> Result<Created<Json<Product>>, (Status, Json<ApiError>)> {
	if let Err(e) = product.validate() {
		return Err(ApiError::validation_error(e))
	}

	let inventory = match ProductInventory::create(
			ProductInventoryInsert::new(0),
			db_inventory
		)
		.await
		.map_err(|e| ApiError::internal_server_error(&e)) {
		Ok(i) => i,
		Err(e) => return Err(e),
	};

	product.inventory_id = Some(inventory.id);


	Product::create(product.into_inner(), db)
		.await
		.map(|response| Created::new(format!("/product/{}", response.id)).body(Json(response)))
		.map_err(|e| ApiError::internal_server_error(&e))
}

#[patch("/product/<id>", format = "json", data = "<product>")]
pub async fn update(
	db: Connection<Db>,
	id: i64,
	product: Json<ProductUpdate>
) -> Result<Json<Product>, (Status, Json<ApiError>)> {
	if let Err(e) = product.validate() {
		return Err(ApiError::validation_error(e))
	}

	Product::update(id, product.into_inner(), db)
		.await
		.map(Json)
		.map_err(|e| ApiError::internal_server_error(&e))
}

#[delete("/product/<id>")]
pub async fn delete(
	db: Connection<Db>,
	id: i64
) -> Result<Json<u64>, (Status, Json<ApiError>)> {
	Product::delete(id, db)
		.await
		.map(Json)
		.map_err(|e| ApiError::internal_server_error(&e))
}