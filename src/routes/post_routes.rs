use rocket::Route;
use rocket::serde::json::Json;

use crate::controllers::post_controller;
use crate::models::post::Post;

pub fn routes() -> Vec<Route> {
    routes![create_post, get_post, get_all_posts]
}

#[post("/", data = "<post>")]
pub fn create_post(post: Json<Post>) -> Result<rocket::http::Status, rocket::http::Status> {
    post_controller::create_post(post.into_inner())
}

#[get("/<post_id>")]
pub fn get_post(post_id: i32) -> Option<Json<Post>> {
    post_controller::get_post(post_id)
}

#[get("/")]
pub fn get_all_posts() -> Json<Vec<Post>> {
    post_controller::get_all_posts()
}
