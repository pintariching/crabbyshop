use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use validator::Validate;

use crate::models::{MAX_I32_CONST, MIN_I32_CONST};

#[derive(Serialize, Deserialize)]
pub struct ProductInventory {
    pub id: i64,
    pub quantity: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Validate)]
pub struct ProductInventoryInsert {
    #[validate(
        required(message = "this field is required"),
        range(
            min = "MIN_I32_CONST",
            max = "MAX_I32_CONST",
            message = "field contains invalid value - min: 0, max: i32"
        )
    )]
    quantity: Option<i32>,
}

impl ProductInventoryInsert {
    pub fn new(quantity: i32) -> ProductInventoryInsert {
        ProductInventoryInsert {
            quantity: Some(quantity),
        }
    }
}

#[derive(Deserialize, Validate)]
pub struct ProductInventoryUpdate {
    #[validate(range(
        min = "MIN_I32_CONST",
        max = "MAX_I32_CONST",
        message = "field contains invalid value - min: 0, max: i32"
    ))]
    quantity: Option<i32>,
}

impl ProductInventory {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<ProductInventory>, String> {
        sqlx::query_as!(
            ProductInventory,
            r#"
				SELECT id, quantity, created_at, updated_at FROM product_inventory ORDER BY id ASC;
			"#
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn find_by_id(id: i64, pool: &PgPool) -> Result<ProductInventory, String> {
        sqlx::query_as!(
            ProductInventory,
            r#"
				SELECT id, quantity, created_at, updated_at FROM product_inventory WHERE id = $1;
			"#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn create(
        input: ProductInventoryInsert,
        pool: &PgPool,
    ) -> Result<ProductInventory, String> {
        sqlx::query_as!(
            ProductInventory,
            r#"
				INSERT INTO product_inventory(quantity)
				VALUES ($1)
				RETURNING id, quantity, created_at, updated_at; 
			"#,
            input.quantity
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn update(
        id: i64,
        input: ProductInventoryUpdate,
        pool: &PgPool,
    ) -> Result<ProductInventory, String> {
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
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn delete(id: i64, pool: &PgPool) -> Result<u64, String> {
        sqlx::query!(
            r#"
				DELETE FROM product_inventory WHERE id = $1;
			"#,
            id
        )
        .execute(pool)
        .await
        .map(|r| r.rows_affected())
        .map_err(|e| e.to_string())
    }
}
