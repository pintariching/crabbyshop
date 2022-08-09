use serde::Deserialize;

pub mod category;
pub mod discount;
pub mod product;
pub mod product_inventory;

#[derive(Deserialize)]
pub struct Params {
    category_id: Option<i64>,
}
