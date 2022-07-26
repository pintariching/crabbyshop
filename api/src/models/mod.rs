use sqlx::types::Decimal;
use validator::ValidationError;

pub mod product;
pub mod category;
pub mod discount;
pub mod product_inventory;
pub mod authentication;

const MIN_I64_CONST: i64 = 0;
const MAX_I64_CONST: i64 = i64::MAX;
const MIN_I32_CONST: i32 = 0;
const MAX_I32_CONST: i32 = i32::MAX;
const TOO_MANY_CHARACTERS_128: &str = "field contains too many characters - max: 128";
const TOO_MANY_CHARACTERS_500: &str = "field contains too many characters - max: 500";
const I64_ERROR: &str = "field contains invalid value - min: 0, max: i64";
const I32_ERROR: &str = "field contains invalid value - min: 0, max: 2147483647";
const REQUIRED: &str = "this field is required";

fn validate_decimal(value: &Decimal) -> Result<(), ValidationError> {
	if *value < Decimal::ZERO {
		return Err(ValidationError::new("range"))
	}

	Ok(())
}