// post_routes.rs

use rocket::http::Status;
use rocket::Route;
use rocket::serde::json::Json;
use crate::controllers::post_controller;
use crate::models::post::Post;
use sqlx::mysql::MySqlPool;

pub fn routes() -> Vec<Route> {
    routes![create_post, get_post, get_all_posts]
}

#[get("/posts")]
pub async fn get_all_posts(pool: &rocket::State<MySqlPool>) -> Result<Json<Vec<Post>>, Status> {
    post_controller::get_all_posts(pool).await
}

#[get("/posts/<post_id>")]
pub async fn get_post(pool: &rocket::State<MySqlPool>, post_id: i32) -> Result<Json<Post>, Status> {
    post_controller::get_post(pool, post_id).await
}

#[post("/posts", format = "json", data = "<post>")]
pub async fn create_post(pool: &rocket::State<MySqlPool>, post: Json<Post>) -> Result<Status, Status> {
    post_controller::create_post(pool, post).await
}
