use chrono::NaiveDateTime;
use rocket_db_pools::Connection;
use serde::{Serialize, Deserialize};
use sqlx::{types::Decimal, FromRow};
use validator::Validate;

use crate::db::Db;

use super::{
	MIN_I64_CONST, 
	MAX_I64_CONST,
	TOO_MANY_CHARACTERS_128,
	TOO_MANY_CHARACTERS_500,
	I64_ERROR,
	REQUIRED,
	validate_decimal
};

use super::{
	category::Category, 
	product_inventory::ProductInventory, 
	discount::Discount
};	

#[derive(Serialize, FromRow)]
pub struct Product {
	pub id: i64,
	name: String,
	description: Option<String>,
	sku: Option<String>,
	category_id: Option<i64>,
	category: Option<Category>,
	inventory_id: Option<i64>,
	inventory: Option<ProductInventory>,
	price: Option<Decimal>,
	discount_id: Option<i64>,
	discount: Option<Discount>,
	created_at: NaiveDateTime,
	updated_at: NaiveDateTime
}

struct ProductFlat {
	id: i64,
	name: String,
	description: Option<String>,
	sku: Option<String>,
	price: Option<Decimal>,

	category_id: Option<i64>,
	category_name: Option<String>,
	category_parent_id: Option<i64>,

	inventory_id: Option<i64>,
	inventory_quantity: Option<i32>,
	inventory_created_at: Option<NaiveDateTime>,
	inventory_updated_at: Option<NaiveDateTime>,

	discount_id: Option<i64>,
	discount_name: Option<String>,
	discount_description: Option<String>,
	discount_percent: Option<Decimal>,
	discount_active: Option<bool>,
	discount_created_at: Option<NaiveDateTime>,
	discount_updated_at: Option<NaiveDateTime>,

	created_at: NaiveDateTime,
	updated_at: NaiveDateTime
}

#[derive(Deserialize, Validate)]
pub struct ProductInsert {
	#[validate(required(message = "CONST:REQUIRED"), length(max = 128, message = "CONST:TOO_MANY_CHARACTERS_128"))]
	name: Option<String>,

	#[validate(length(max = 500, message = "CONST:TOO_MANY_CHARACTERS_500"))]
	description: Option<String>,

	#[validate(length(max = 128, message = "CONST:TOO_MANY_CHARACTERS_128"))]
	sku: Option<String>,

	#[validate(range(min = "MIN_I64_CONST", max = "MAX_I64_CONST", message = "CONST:I64_ERROR"))]
	category_id: Option<i64>,

	#[serde(skip_serializing)]
	pub inventory_id: Option<i64>,

	#[validate(range(min = "MIN_I64_CONST", max = "MAX_I64_CONST", message = "CONST:I64_ERROR"))]
	discount_id: Option<i64>,
	
	#[validate(custom(function = "validate_decimal", message ="CONST:I64_ERROR"))]
	price: Option<Decimal>,
}

#[derive(Deserialize, Validate)]
pub struct ProductUpdate {
	#[validate(length(max = 128, message = "CONST:TOO_MANY_CHARACTERS_128"))]
	name: Option<String>,

	#[validate(length(max = 500, message = "CONST:TOO_MANY_CHARACTERS_500"))]
	description: Option<String>,

	#[validate(length(max = 128, message = "CONST:TOO_MANY_CHARACTERS_128"))]
	sku: Option<String>,

	#[validate(range(min = "MIN_I64_CONST", max = "MAX_I64_CONST", message = "CONST:I64_ERROR"))]
	category_id: Option<i64>,

	#[validate(range(min = "MIN_I64_CONST", max = "MAX_I64_CONST", message = "CONST:I64_ERROR"))]
	inventory_id: Option<i64>,

	#[validate(range(min = "MIN_I64_CONST", max = "MAX_I64_CONST", message = "CONST:I64_ERROR"))]
	discount_id: Option<i64>,
	
	#[validate(custom(function = "validate_decimal", message ="CONST:I64_ERROR"))]
	price: Option<Decimal>,
}

impl Product {
	fn from_flat(p: ProductFlat) -> Product {
		let mut product = Product {
			id: p.id,
			name: p.name,
			description: p.description,
			sku: p.sku,
			category_id: p.category_id,
			category: None,
			inventory_id: p.inventory_id,
			inventory: None,
			price: p.price,
			discount_id: p.discount_id,
			discount: None,
			created_at: p.created_at,
			updated_at: p.updated_at
		};

		if let (Some(id), Some(name)) = (p.category_id, p.category_name) {
			product.category = Some(Category {
				id,
				name,
				parent_id: p.category_parent_id 
			});
		}

		if let (Some(id), Some(quantity), Some(created_at), Some(updated_at)) = 
			(p.inventory_id, p.inventory_quantity, p.inventory_created_at, p.inventory_updated_at) {
			product.inventory = Some(ProductInventory {
				id,
				quantity,
				created_at,
				updated_at
			})
		}

		if let (Some(id), Some(name), Some(active), Some(created_at), Some(updated_at)) = 
			(p.discount_id, p.discount_name, p.discount_active, p.discount_created_at, p.discount_updated_at) {
			product.discount = Some(Discount {
				id,
				name,
				description: p.discount_description,
				discount_percent: p.discount_percent,
				active,
				created_at,
				updated_at
			})
		}

		product
	}

	pub async fn find_all(mut pool: Connection<Db>) -> Result<Vec<Product>, String> {
		sqlx::query_as!(
			ProductFlat,
			r#"
				SELECT * FROM product_flat_view ORDER BY "id!" ASC;
			"#
		)
		.fetch_all(&mut *pool)
		.await
		.map(|p|
			p.into_iter().map(|f| Product::from_flat(f)).collect::<Vec<Product>>()
		)
		.map_err(|e| e.to_string())
	}

	pub async fn find_by_id(id: i64, mut pool: Connection<Db>) -> Result<Product, String> {
		sqlx::query_as!(
			ProductFlat,
			r#"
				SELECT * FROM product_flat_view WHERE "id!" = $1;
			"#,
			id
		)
		.fetch_one(&mut *pool)
		.await
		.map(|f| Product::from_flat(f))
		.map_err(|e| e.to_string())
	}

	pub async fn find_by_category(category_id: i64, mut pool: Connection<Db>) -> Result<Vec<Product>, String> {
		sqlx::query_as!(
			ProductFlat,
			r#"
				SELECT * FROM product_flat_view WHERE category_id = $1 ORDER BY "id!" ASC;
			"#,
			category_id
		)
		.fetch_all(&mut *pool)
		.await
		.map(|p|
			p.into_iter().map(|f| Product::from_flat(f)).collect::<Vec<Product>>()
		)
		.map_err(|e| e.to_string())
	}

	pub async fn create(input: ProductInsert, mut pool: Connection<Db>) -> Result<Product, String> {
		let id = match sqlx::query!(
			r#"
				INSERT INTO product (name, description, sku, category_id, price, discount_id, inventory_id)
				VALUES ($1, $2, $3, $4, $5, $6, $7)
				RETURNING id;
			"#,
			input.name,
			input.description,
			input.sku,
			input.category_id,
			input.price,
			input.discount_id,
			input.inventory_id
		)
		.fetch_one(&mut *pool)
		.await
		.map(|r| r.id)
		.map_err(|e| e.to_string()) {
			Ok(id) => id,
			Err(e) => return Err(e),
		};
	
		Product::find_by_id(id, pool).await
	}

	pub async fn update(id: i64, input: ProductUpdate, mut pool: Connection<Db>) -> Result<Product, String> {
		let id = match sqlx::query!(
			r#"
				UPDATE product SET 
					name = COALESCE($1, name),
					description = COALESCE($2, description),
					sku = COALESCE($3, sku),
					category_id = COALESCE($4, category_id),
					price = COALESCE($5, price),
					discount_id = COALESCE($6, discount_id),
					inventory_id = COALESCE($7, inventory_id),
					updated_at = NOW()
				WHERE id = $8
				RETURNING id;
			"#,
			input.name,
			input.description,
			input.sku,
			input.category_id,
			input.price,
			input.discount_id,
			input.inventory_id,
			id
		)
		.fetch_one(&mut *pool)
		.await
		.map(|r| r.id)
		.map_err(|e| e.to_string()) {
			Ok(id) => id,
			Err(e) => return Err(e),
		};

		Product::find_by_id(id, pool).await
	}

	pub async fn delete(id: i64, mut pool: Connection<Db>) -> Result<u64, String> {
		sqlx::query!(
			r#"
				DELETE FROM product WHERE id = $1;
			"#,
			id
		)
		.execute(&mut *pool)
		.await
		.map(|r| r.rows_affected())
		.map_err(|e| e.to_string())
	}
}