use std::env;
use dotenv::dotenv;
use rocket::{figment::{value::{Map, Value}, util::map, Figment}, fairing};
use rocket::{Rocket, Build, fairing::AdHoc};
use rocket_db_pools::Database;

#[derive(Database)]
#[database("sqlx_postgres_pool")]
pub struct Db(sqlx::PgPool);


pub async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
	match Db::fetch(&rocket) {
		Some(db) => match sqlx::migrate!().run(&**db).await {
			Ok(_) => Ok(rocket),
			Err(e) => {
				error!("Failed to initialize SQLx database: {}", e);
				Err(rocket)
			}
		},
		None => Err(rocket),
	}
}

pub fn stage() -> AdHoc {
	AdHoc::on_ignite("SQLx Stage", |rocket| async {
		rocket
			.attach(Db::init())
			.attach(AdHoc::try_on_ignite("SQLx migrations", run_migrations))
	})
}

pub fn from_env() -> Figment {
	dotenv().ok();

	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable not found");

	let database: Map<_, Value> = map! {
		"url" => database_url.into(),
		"pool_size" => 10.into()
	};
	
	rocket::Config::figment().merge(("databases", map!["sqlx_postgres_pool" => database]))
}