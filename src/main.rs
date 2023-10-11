#[macro_use]
extern crate rocket;
use tokio;

mod db;
mod routes;
mod models;
mod controllers;

#[launch]
fn rocket() -> _ {
    let pool = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(db::init_pool())
        .expect("Failed to create database pool");

    rocket::build()
        .manage(pool)
        .mount("/api/posts", routes::post_routes::routes())
}
