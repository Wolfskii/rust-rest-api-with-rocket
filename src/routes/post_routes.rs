use rocket::http::Status;
use rocket::Route;
use rocket::data::Data;
use rocket::serde::json::Json;

use crate::controllers::post_controller;
use crate::models::post::Post;

pub fn routes() -> Vec<Route> {
    routes![create_post, get_post, get_all_posts]
}

#[get("/")]
pub fn get_all_posts() -> Json<Vec<Post>> {
    post_controller::get_all_posts()
}

#[get("/<post_id>")]
pub fn get_post(post_id: i32) -> Option<Json<Post>> {
    post_controller::get_post(post_id)
}

#[post("/", data = "<_post>")]
pub async fn create_post(_post: Data<'_>) -> Result<Status, Status> {
    post_controller::create_post(_post).await
}
