use chrono::NaiveDateTime;
use rocket_db_pools::Connection;
use serde::{Serialize, Deserialize};
use sqlx::{types::Decimal, FromRow};
use validator::Validate;

use crate::db::Db;
use super::{
	TOO_MANY_CHARACTERS_128,
	TOO_MANY_CHARACTERS_500,
	I64_ERROR,
	REQUIRED,
	validate_decimal
};

#[derive(Serialize, FromRow)]
pub struct Discount {
	pub id: i64,
	pub name: String,
	pub description: Option<String>,
	pub discount_percent: Option<Decimal>,
	pub active: bool,
	pub created_at: NaiveDateTime,
	pub updated_at: NaiveDateTime
}

#[derive(Deserialize, Validate)]
pub struct DiscountInsert {
	#[validate(required(message = "CONST:REQUIRED"), length(max = 128, message = "CONST:TOO_MANY_CHARACTERS_128"))]
	name: Option<String>,

	#[validate(length(max = 128))]
	description: Option<String>,

	#[validate(custom(function = "validate_decimal", message ="CONST:I64_ERROR"))]
	discount_percent: Option<Decimal>,

	#[serde(default)]
	active: bool
}

#[derive(Deserialize, Validate)]
pub struct DiscountUpdate {
	#[validate(length(max = 128, message = "CONST:TOO_MANY_CHARACTERS_128"))]
	name: Option<String>,

	#[validate(length(max = 500, message = "CONST:TOO_MANY_CHARACTERS_500"))]
	description: Option<String>,
	
	#[validate(custom(function = "validate_decimal", message ="CONST:I64_ERROR"))]
	discount_percent: Option<Decimal>,

	active: Option<bool>
}

impl Discount {
	pub async fn find_all(mut pool: Connection<Db>) -> Result<Vec<Discount>, String> {
		sqlx::query_as!(
			Discount,
			r#"
				SELECT id, name, description, discount_percent, active, created_at, updated_at FROM discount ORDER BY id ASC;
			"#
		)
		.fetch_all(&mut *pool)
		.await
		.map_err(|e| e.to_string())
	}

	pub async fn find_by_id(id: i64, mut pool: Connection<Db>) -> Result<Discount, String> {
		sqlx::query_as!(
			Discount,
			r#"
				SELECT id, name, description, discount_percent, active, created_at, updated_at FROM discount WHERE id = $1;
			"#,
			id
		)
		.fetch_one(&mut *pool)
		.await
		.map_err(|e| e.to_string())
	}

	pub async fn create(input: DiscountInsert, mut pool: Connection<Db>) -> Result<Discount, String> {
		sqlx::query_as!(
			Discount,
			r#"
				INSERT INTO discount(name, description, discount_percent, active)
				VALUES ($1, $2, $3, $4)
				RETURNING id, name, description, discount_percent, active, created_at, updated_at; 
			"#,
			input.name,
			input.description,
			input.discount_percent,
			input.active
		)
		.fetch_one(&mut *pool)
		.await
		.map_err(|e| e.to_string())
	}

	pub async fn set_active(id: i64, mut pool: Connection<Db>) -> Result<u64, String> {
		sqlx::query_as!(
			Discount,
			r#"UPDATE discount SET active = true, updated_at = NOW() WHERE id = $1;"#,
			id 
		)
		.execute(&mut *pool)
		.await
		.map(|r| r.rows_affected())
		.map_err(|e| e.to_string())
	}

	pub async fn set_inactive(id: i64, mut pool: Connection<Db>) -> Result<u64, String> {
		sqlx::query_as!(
			Discount,
			r#"UPDATE discount SET active = false, updated_at = NOW() WHERE id = $1;"#,
			id 
		)
		.execute(&mut *pool)
		.await
		.map(|r| r.rows_affected())
		.map_err(|e| e.to_string())
	}

	pub async fn update(id: i64, input: DiscountUpdate, mut pool: Connection<Db>) -> Result<Discount, String> {
		sqlx::query_as!(
			Discount,
			r#"
				UPDATE discount SET 
					name = COALESCE($1, name),
					description = COALESCE($2, description),
					discount_percent = COALESCE($3, discount_percent),
					active = COALESCE($4, active),
					updated_at = NOW()
				WHERE id = $5
				RETURNING id, name, description, discount_percent, active, created_at, updated_at;
			"#,
			input.name,
			input.description,
			input.discount_percent,
			input.active,
			id
		)
		.fetch_one(&mut *pool)
		.await
		.map_err(|e| e.to_string())
	}

	pub async fn delete(id: i64, mut pool: Connection<Db>) -> Result<u64, String> {
		sqlx::query!(
			r#"
				DELETE FROM discount WHERE id = $1;
			"#,
			id
		)
		.execute(&mut *pool)
		.await
		.map(|r| r.rows_affected())
		.map_err(|e| e.to_string())
	}
}