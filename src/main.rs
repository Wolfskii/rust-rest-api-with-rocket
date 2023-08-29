#[macro_use] extern crate rocket;

mod routes;

#[launch]
fn rocket() -> _ {
        rocket::build()
        .mount("/api/posts", routes::post_routes::routes())
}
