use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use validator::Validate;

#[derive(Serialize, Clone, Debug)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub parent_id: Option<i64>,
    pub children: Vec<Category>,
}

#[derive(FromRow, Clone, Debug)]
pub struct CategoryDb {
    pub id: i64,
    pub name: String,
    pub parent_id: Option<i64>,
}

#[derive(Deserialize, Validate)]
pub struct CategoryInsert {
    #[validate(
        required(message = "this field is required"),
        length(max = 128, message = "field contains too many characters - max: 128")
    )]
    name: Option<String>,
    parent_id: Option<i64>,
}

#[derive(Deserialize, Validate)]
pub struct CategoryUpdate {
    #[validate(length(max = 128, message = "CONST:TOO_MANY_CHARACTERS_128"))]
    name: Option<String>,

    parent_id: Option<i64>,
}

impl Category {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<Category>, String> {
        sqlx::query_as!(
            CategoryDb,
            r#"
				SELECT id, name, parent_id FROM category ORDER BY id ASC;
			"#
        )
        .fetch_all(pool)
        .await
        .map(|mut c| sort_categories(&mut c))
        .map_err(|e| e.to_string())
    }

    pub async fn find_by_id(id: i64, pool: &PgPool) -> Result<Vec<Category>, String> {
        sqlx::query_as_unchecked!(
            CategoryDb,
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
        .fetch_all(pool)
        .await
        .map(|mut c| sort_categories(&mut c))
        .map_err(|e| e.to_string())
    }

    pub async fn create(input: CategoryInsert, pool: &PgPool) -> Result<Category, String> {
        sqlx::query_as!(
            CategoryDb,
            r#"
				INSERT INTO category(name, parent_id) VALUES ($1, $2)
				RETURNING id, name, parent_id; 
			"#,
            input.name,
            input.parent_id
        )
        .fetch_one(pool)
        .await
        .map(|c| Category::from_db(&c))
        .map_err(|e| e.to_string())
    }

    pub async fn update(id: i64, input: CategoryUpdate, pool: &PgPool) -> Result<Category, String> {
        sqlx::query_as!(
            CategoryDb,
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
        .fetch_one(pool)
        .await
        .map(|c| Category::from_db(&c))
        .map_err(|e| e.to_string())
    }

    pub async fn delete(id: i64, pool: &PgPool) -> Result<u64, String> {
        sqlx::query!(
            r#"
				DELETE FROM category WHERE id = $1;
			"#,
            id
        )
        .execute(pool)
        .await
        .map(|result| result.rows_affected())
        .map_err(|e| e.to_string())
    }

    fn from_db(c: &CategoryDb) -> Category {
        Category {
            id: c.id,
            name: c.name.clone(),
            parent_id: c.parent_id,
            children: Vec::new(),
        }
    }
}

fn sort_categories(categories: &mut Vec<CategoryDb>) -> Vec<Category> {
    let mut root_categories = categories
        .iter()
        .filter(|c| c.parent_id == None)
        .map(|c| Category::from_db(c))
        .collect::<Vec<Category>>();

    for cat in root_categories.iter_mut() {
        recursive_sort(cat, categories);
    }

    root_categories
}

// TODO: this needs to be optimized
fn recursive_sort(root_category: &mut Category, all_categories: &mut Vec<CategoryDb>) {
    for category in all_categories.iter() {
        if let Some(parent_id) = category.parent_id {
            if parent_id == root_category.id {
                root_category.children.push(Category::from_db(category));
            }
        }
    }

    let mut new_categories = all_categories.clone();
    for child_category in root_category.children.iter_mut() {
        new_categories.swap_remove(
            new_categories
                .iter()
                .position(|c| c.id == child_category.id)
                .unwrap(),
        );
    }

    if new_categories.is_empty() {
        return;
    }

    for child_category in root_category.children.iter_mut() {
        recursive_sort(child_category, &mut new_categories);
    }
}
