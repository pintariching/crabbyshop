use jsonwebtoken::DecodingKey;
use rocket::fairing::AdHoc;

pub struct JwtPublicKey(pub DecodingKey);

pub fn stage() -> AdHoc {
	AdHoc::on_ignite("Authentication stage", |rocket| async {
		rocket
			.manage(JwtPublicKey(DecodingKey::from_rsa_pem(include_bytes!("../jwtRSA256-public.pem"))
					.expect("error extracting JWT public key"))
			)
	})
}