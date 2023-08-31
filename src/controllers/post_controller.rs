use rocket::http::Status;
use rocket::data::Data;
use rocket::serde::json::Json;

use crate::models::post::Post;

pub fn get_all_posts() -> Json<Vec<Post>> {
    let example_posts = vec![
        // Example posts
        Post {
            id: 1,
            title: "Example Post 1".to_string(),
            content: "This is the content of example post 1.".to_string(),
        },
        Post {
            id: 2,
            title: "Example Post 2".to_string(),
            content: "This is the content of example post 2.".to_string(),
        }
    ];

    Json(example_posts)
}

pub fn get_post(_post_id: i32) -> Option<Json<Post>> {
    // Implement your get_post logic here
    None
}

pub async fn create_post(_post: Data<'_>) -> Result<Status, Status> {
    // Implement your create_post logic here
    Ok(Status::Created)
}
