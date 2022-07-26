use chrono::NaiveDateTime;
use rocket_db_pools::Connection;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use validator::Validate;

use crate::db::Db;

use super::{
	MIN_I32_CONST,
	MAX_I32_CONST,
	I32_ERROR,
	REQUIRED
};

#[derive(Serialize, FromRow)]
pub struct ProductInventory {
	pub id: i64,
	pub quantity: i32,
	pub created_at: NaiveDateTime,
	pub updated_at: NaiveDateTime
}

#[derive(Deserialize, Validate)]
pub struct ProductInventoryInsert {
	#[validate(required(message = "CONST:REQUIRED"), range(min = "MIN_I32_CONST", max = "MAX_I32_CONST", message = "CONST:I32_ERROR"))]
	quantity: Option<i32>
}

impl ProductInventoryInsert {
	pub fn new(quantity: i32) -> ProductInventoryInsert {
		ProductInventoryInsert { quantity: Some(quantity) }
	}
}

#[derive(Deserialize, Validate)]
pub struct ProductInventoryUpdate {
	#[validate(range(min = "MIN_I32_CONST", max = "MAX_I32_CONST", message = "CONST:I32_ERROR"))]
	quantity: Option<i32>
}

impl ProductInventory {
	pub async fn find_all(mut pool: Connection<Db>) -> Result<Vec<ProductInventory>, String> {
		sqlx::query_as!(
			ProductInventory,
			r#"
				SELECT id, quantity, created_at, updated_at FROM product_inventory ORDER BY id ASC;
			"#
		)
		.fetch_all(&mut *pool)
		.await
		.map_err(|e| e.to_string())
	}

	pub async fn find_by_id(id: i64, mut pool: Connection<Db>) -> Result<ProductInventory, String> {
		sqlx::query_as!(
			ProductInventory,
			r#"
				SELECT id, quantity, created_at, updated_at FROM product_inventory WHERE id = $1;
			"#,
			id
		)
		.fetch_one(&mut *pool)
		.await
		.map_err(|e| e.to_string())
	}

	pub async fn create(input: ProductInventoryInsert, mut pool: Connection<Db>) -> Result<ProductInventory, String> {
		sqlx::query_as!(
			ProductInventory,
			r#"
				INSERT INTO product_inventory(quantity)
				VALUES ($1)
				RETURNING id, quantity, created_at, updated_at; 
			"#,
			input.quantity
		)
		.fetch_one(&mut *pool)
		.await
		.map_err(|e| e.to_string())
	}

	pub async fn update(id: i64, input: ProductInventoryUpdate, mut pool: Connection<Db>) -> Result<ProductInventory, String> {
		sqlx::query_as!(
			ProductInventory,
			r#"
				UPDATE product_inventory SET 
					quantity = COALESCE($1, quantity),
					updated_at = NOW()
				WHERE id = $2
				RETURNING id, quantity, created_at, updated_at;
			"#,
			input.quantity,
			id
		)
		.fetch_one(&mut *pool)
		.await
		.map_err(|e| e.to_string())
	}

	pub async fn delete(id: i64, mut pool: Connection<Db>) -> Result<u64, String> {
		sqlx::query!(
			r#"
				DELETE FROM product_inventory WHERE id = $1;
			"#,
			id
		)
		.execute(&mut *pool)
		.await
		.map(|r| r.rows_affected())
		.map_err(|e| e.to_string())
	}
}