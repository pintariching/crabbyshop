use jsonwebtoken::{TokenData, DecodingKey, Validation};
use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use serde::{Deserialize, Serialize};
use crate::auth::JwtPublicKey;

#[derive(Serialize, Deserialize)]
pub struct JwtToken {
	iat: i64,
	exp: i64,
	user_role: Vec<String>
}

impl JwtToken {
	pub fn is_admin(&self) -> bool {
		self.user_role.contains(&"admin".to_string())
	}
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JwtToken {
	type Error = String;

	async fn from_request(request: &'r Request<'_>) ->  Outcome<Self, Self::Error> {
		let secrets = request.rocket().state::<JwtPublicKey>().unwrap();

		let auth_header = match request.headers().get_one("Authorization") {
			Some(a) => a,
			None => return return_outcome("Authorization header not found"),
		};

		let auth_string = auth_header.to_string();
		if auth_string.starts_with("Bearer") {
			let token = auth_string[6..auth_string.len()].trim();
			match decode_token(token, &secrets.0) {
				Ok(t) => Outcome::Success(t.claims),
				Err(e) => return_outcome(&e),
			}
		} else {
			return return_outcome("Authentication string does not containt 'Bearer'")
		}
	}
}

fn decode_token(token: &str, public_key: &DecodingKey) -> Result<TokenData<JwtToken>, String> {
	match jsonwebtoken::decode::<JwtToken>(&token, public_key, &Validation::new(jsonwebtoken::Algorithm::RS256)) {
		Ok(t) => Ok(t),
		Err(e) => Err(e.to_string()),
	}
}

fn return_outcome(reason: &str) -> Outcome<JwtToken, String> {
	Outcome::Failure((
		Status::BadRequest,
		reason.to_string()
	))
}