use rocket::http::Status;
use rocket::data::Data;
use rocket::serde::json::Json;

use crate::models::post::Post;

pub async fn create_post(_post: Data<'_>) -> Result<Status, Status> {
    // Implement your create_post logic here
    Ok(Status::Created)
}

pub fn get_post(post_id: i32) -> Option<Json<Post>> {
    // Implement your get_post logic here
    None
}

pub fn get_all_posts() -> Json<Vec<Post>> {
    // Implement your get_all_posts logic here
    Json(vec![])
}
