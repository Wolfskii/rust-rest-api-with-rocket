#[macro_use] extern crate rocket;
use dotenv::dotenv;
use std::env;

mod routes;
mod models;
mod controllers;

#[launch]
fn rocket() -> _ {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

        rocket::build()
        .mount("/api/posts", routes::post_routes::routes())
}
