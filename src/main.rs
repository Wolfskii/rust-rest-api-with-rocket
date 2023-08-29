#[macro_use] extern crate rocket;

mod routes;
mod models;
mod controllers;

#[launch]
fn rocket() -> _ {
        rocket::build()
        .mount("/api/posts", routes::post_routes::routes())
}
