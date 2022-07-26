use rocket_db_pools::Connection;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use validator::Validate;

use crate::db::Db;

use super::{
	MIN_I64_CONST, 
	MAX_I64_CONST,
	TOO_MANY_CHARACTERS_128,
	I64_ERROR,
	REQUIRED,
};

#[derive(Serialize, FromRow, Debug, Clone)]
pub struct Category {
	pub id: i64,
	pub name: String,
	pub parent_id: Option<i64>
}

#[derive(Deserialize, Validate)]
pub struct CategoryInsert {
	#[validate(required(message = "CONST:REQUIRED"), length(max = 128, message = "CONST:TOO_MANY_CHARACTERS_128"))]
	name: Option<String>,

	#[validate(range(min = "MIN_I64_CONST", max = "MAX_I64_CONST", message = "CONST:I64_ERROR"))]
	parent_id: Option<i64>
}

#[derive(Deserialize, Validate)]
pub struct CategoryUpdate {
	#[validate(length(max = 128, message = "CONST:TOO_MANY_CHARACTERS_128"))]
	name: Option<String>,

	#[validate(range(min = "MIN_I64_CONST", max = "MAX_I64_CONST", message = "CONST:I64_ERROR"))]
	parent_id: Option<i64>
}

impl Category {
	pub async fn find_all(mut pool: Connection<Db>) -> Result<Vec<Category>, String> {
		sqlx::query_as!(
			Category,
			r#"
				SELECT id, name, parent_id FROM category ORDER BY id ASC;
			"#
		)
		.fetch_all(&mut *pool)
		.await
		.map_err(|e| e.to_string())
	}
	
	pub async fn find_by_id(id: i64, mut pool: Connection<Db>) -> Result<Vec<Category>, String> {
		sqlx::query_as_unchecked!(
			Category,
			r#"
				WITH RECURSIVE category_tree AS 
				(
					SELECT c1.id, c1.name, c1.parent_id FROM category c1
					WHERE c1.id = $1
					
					UNION ALL
					
					SELECT c2.id, c2.name, c2.parent_id FROM category c2
					JOIN category_tree ct ON ct.parent_id = c2.id 
				)
				SELECT id, name, parent_id FROM category_tree;
			"#,
			id
		)
		.fetch_all(&mut *pool)
		.await
		.map_err(|e| e.to_string())
	}
	
	pub async fn create(input: CategoryInsert, mut pool: Connection<Db>) -> Result<Category, String> {
		sqlx::query_as!(
			Category,
			r#"
				INSERT INTO category(name, parent_id) VALUES ($1, $2)
				RETURNING id, name, parent_id; 
			"#,
			input.name,
			input.parent_id
		)
		.fetch_one(&mut *pool)
		.await
		.map_err(|e| e.to_string())
	}
	
	pub async fn update(id: i64, input: CategoryUpdate, mut pool: Connection<Db>) -> Result<Category, String> {
		sqlx::query_as!(
			Category,
			r#"
				UPDATE category SET 
					name = COALESCE($1, name),
					parent_id = COALESCE($2, parent_id)
				WHERE id = $3
				RETURNING id, name, parent_id;
			"#,
			input.name,
			input.parent_id,
			id
		)
		.fetch_one(&mut *pool)
		.await
		.map_err(|e| e.to_string())
	}

	pub async fn delete(id: i64, mut pool: Connection<Db>) -> Result<u64, String> {
		sqlx::query!(
			r#"
				DELETE FROM category WHERE id = $1;
			"#,
			id
		)
		.execute(&mut *pool)
		.await
		.map(|result| result.rows_affected())
		.map_err(|e| e.to_string())
	}
}

#[derive(Serialize)]
pub struct CategorySorted {
	id: i64,
	name: String,
	parent_id: Option<i64>,
	children: Vec<CategorySorted>
}

impl CategorySorted {
	pub fn from_categories(categories: Vec<Category>) -> Option<Vec<CategorySorted>> {
		let mut root_categories: Vec<CategorySorted> =
			categories.iter()
				.filter(|c| c.parent_id == None)
				.map(|c| CategorySorted::from_category(c.clone()))
				.collect();

		//
		// TODO!: this mess is ugly and needs to be refactored
		//
		for root_category in root_categories.iter_mut() {
			for cat1 in categories.iter().filter(|c| c.parent_id == Some(root_category.id)) {
				let mut sub1 = CategorySorted::from_category(cat1.clone());
				
				for cat2 in categories.iter().filter(|c| c.parent_id == Some(cat1.id)) {
					let mut sub2 = CategorySorted::from_category(cat2.clone());
					
					for cat3 in categories.iter().filter(|c| c.parent_id == Some(cat2.id)) {
						let mut sub3 = CategorySorted::from_category(cat3.clone());
						
						for cat4 in categories.iter().filter(|c| c.parent_id == Some(cat3.id)) {
							let sub4 = CategorySorted::from_category(cat4.clone());
							sub3.children.push(sub4);
						}
						sub2.children.push(sub3);
					}
					sub1.children.push(sub2);
				}
				root_category.children.push(sub1);
			}
		}
		
		Some(root_categories)
	}

	fn from_category(category: Category) -> CategorySorted {
		CategorySorted { 
			id: category.id,
			name: category.name,
			parent_id: category.parent_id,
			children: Vec::new()
		}
	}

	

}