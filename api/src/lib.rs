#[macro_use] extern crate rocket;

mod db;
mod errors;
mod models;
mod routes;
mod auth;

use rocket::Request;

use crate::routes::{
	category,
	discount,
	product,
	product_inventory
};

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

#[launch]
pub fn rocket() -> _ {
	rocket::custom(db::from_env())
		.attach(db::stage())
		.attach(auth::stage())
		.register("/", catchers![not_found])
		.mount(
			"/api", 
			routes![
				category::fetch_all,
				category::fetch_one,
				category::create,
				category::update,
				category::delete,
				discount::fetch_all,
				discount::fetch_one,
				discount::create,
				discount::set_active,
				discount::set_inactive,
				discount::update,
				discount::delete,
				product::fetch_all,
				product::fetch_one,
				product::fetch_by_category,
				product::create,
				product::update,
				product::delete,
				product_inventory::fetch_all,
				product_inventory::fetch_one,
				product_inventory::create,
				product_inventory::update,
				product_inventory::delete,
			]	
		)
}