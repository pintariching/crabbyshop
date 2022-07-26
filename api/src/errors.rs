use rocket::{http::Status, serde::json::Json, log::private::debug};
use serde::Serialize;
use validator::{ValidationError, ValidationErrors};

#[derive(Serialize, Debug)]
pub struct ApiError {
	pub status: String,
	pub reason: String,
	pub validation_errors: Option<Vec<FieldError>>,
}

#[derive(Serialize, Debug)]
pub struct FieldError {
	pub field: String,
	pub errors: Vec<String>,
}

impl FieldError {
	fn from_validation_error(field: &str, errors: &[ValidationError]) -> FieldError {
		FieldError {
			field: field.to_string(),
			errors: errors.iter()
				.map(|e| if let Some(message) = e.message.clone() {
					debug!("{}", message);
					String::from(message)
				} else {
					String::new()
				})
				.collect()
		}
	}
}

impl ApiError {
	pub fn new(status: Status, reason: String) -> ApiError {
		ApiError { status: status.to_string(), reason, validation_errors: None }
	}

	pub fn new_json(status: Status, reason: String, validation_errors: Option<Vec<FieldError>>) -> Json<ApiError> {
		Json(ApiError { status: status.to_string(), reason, validation_errors })
	}

	pub fn internal_server_error(reason: &str) -> (Status, Json<ApiError>) {
		let status = Status::InternalServerError;

		(status, ApiError::new_json(status, reason.to_string(),None))
	}

	pub fn not_found(reason: &str) -> (Status, Json<ApiError>) {
		let status = Status::NotFound;

		(status, ApiError::new_json(status, reason.to_string(),None))
	}

	pub fn bad_request(reason: &str) -> (Status, Json<ApiError>) {
		let status = Status::BadRequest;

		(status, ApiError::new_json(status, reason.to_string(),None))
	}

	pub fn validation_error(validation_errors: ValidationErrors) -> (Status, Json<ApiError>) {
		let status = Status::BadRequest;

		(
			status,
			ApiError::new_json(
				status,
				String::new(),
					Some(validation_errors
					.field_errors()
					.into_iter()
					.map(|field| 
						FieldError::from_validation_error(field.0, field.1))
					.collect()
					)
			)
		)
	}
}