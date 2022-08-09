use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::types::{Decimal, Json};
use sqlx::PgPool;
use validator::Validate;

use super::{category::CategoryDb, discount::Discount, product_inventory::ProductInventory};

#[derive(Serialize)]
pub struct Product {
    pub id: i64,
    name: String,
    description: Option<String>,
    sku: Option<String>,
    category_id: Option<i64>,
    category: Option<Json<CategoryDb>>,
    inventory_id: Option<i64>,
    inventory: Option<Json<ProductInventory>>,
    price: Option<Decimal>,
    discount_id: Option<i64>,
    discount: Option<Json<Discount>>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Deserialize, Validate)]
pub struct ProductInsert {
    #[validate(
        required(message = "this field is required"),
        length(max = 128, message = "field contains too many characters - max: 128")
    )]
    name: Option<String>,

    #[validate(length(max = 500, message = "field contains too many characters - max: 500"))]
    description: Option<String>,

    #[validate(length(max = 128, message = "field contains too many characters - max: 128"))]
    sku: Option<String>,

    category_id: Option<i64>,

    #[serde(skip_serializing)]
    pub inventory_id: Option<i64>,

    discount_id: Option<i64>,
    price: Option<Decimal>,
}

#[derive(Deserialize, Validate)]
pub struct ProductUpdate {
    #[validate(length(max = 128, message = "field contains too many characters - max: 128"))]
    name: Option<String>,

    #[validate(length(max = 500, message = "field contains too many characters - max: 500"))]
    description: Option<String>,

    #[validate(length(max = 128, message = "field contains too many characters - max: 128"))]
    sku: Option<String>,

    category_id: Option<i64>,
    inventory_id: Option<i64>,
    discount_id: Option<i64>,
    price: Option<Decimal>,
}

impl Product {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<Product>, String> {
        sqlx::query_as!(
            Product,
            r#"
				SELECT p.*, 
					row_to_json(i.*) as "inventory: Json<ProductInventory>",
					row_to_json(d.*) as "discount: Json<Discount>",
					row_to_json(c.*) as "category: Json<CategoryDb>"
				FROM product p
				LEFT JOIN product_inventory i ON i.id = p.inventory_id
				LEFT JOIN discount d ON d.id = p.discount_id
				LEFT JOIN category c on c.id = p.category_id
				ORDER BY id ASC;
			"#
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn find_by_id(id: i64, pool: &PgPool) -> Result<Product, String> {
        sqlx::query_as!(
            Product,
            r#"
				SELECT p.*, 
					row_to_json(i.*) as "inventory: Json<ProductInventory>",
					row_to_json(d.*) as "discount: Json<Discount>",
					row_to_json(c.*) as "category: Json<CategoryDb>"
				FROM product p
				LEFT JOIN product_inventory i ON i.id = p.inventory_id
				LEFT JOIN discount d ON d.id = p.discount_id
				LEFT JOIN category c on c.id = p.category_id
				WHERE p.id = $1;
			"#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn find_by_category(
        category_id: Option<i64>,
        pool: &PgPool,
    ) -> Result<Vec<Product>, String> {
        sqlx::query_as!(
            Product,
            r#"
				SELECT p.*, 
					row_to_json(i.*) as "inventory: Json<ProductInventory>",
					row_to_json(d.*) as "discount: Json<Discount>",
					row_to_json(c.*) as "category: Json<CategoryDb>"
				FROM product p
				LEFT JOIN product_inventory i ON i.id = p.inventory_id
				LEFT JOIN discount d ON d.id = p.discount_id
				LEFT JOIN category c on c.id = p.category_id
				WHERE p.category_id = $1;;
			"#,
            category_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())
    }

    pub async fn create(input: ProductInsert, pool: &PgPool) -> Result<Product, String> {
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
        .fetch_one(pool)
        .await
        .map(|r| r.id)
        .map_err(|e| e.to_string())
        {
            Ok(id) => id,
            Err(e) => return Err(e),
        };

        Product::find_by_id(id, pool).await
    }

    pub async fn update(id: i64, input: ProductUpdate, pool: &PgPool) -> Result<Product, String> {
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
        .fetch_one(pool)
        .await
        .map(|r| r.id)
        .map_err(|e| e.to_string())
        {
            Ok(id) => id,
            Err(e) => return Err(e),
        };

        Product::find_by_id(id, pool).await
    }

    pub async fn delete(id: i64, pool: &PgPool) -> Result<u64, String> {
        sqlx::query!(
            r#"
				DELETE FROM product WHERE id = $1;
			"#,
            id
        )
        .execute(pool)
        .await
        .map(|r| r.rows_affected())
        .map_err(|e| e.to_string())
    }
}
