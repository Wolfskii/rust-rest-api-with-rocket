// post_controller.rs

use rocket::http::Status;
use rocket::serde::json::Json;

use crate::models::post::Post;
use sqlx::mysql::MySqlPool;
use sqlx::query;
use sqlx::query_as;

// Function to get all posts from the database
#[rocket::get("/posts")]
pub async fn get_all_posts(pool: &rocket::State<MySqlPool>) -> Result<Json<Vec<Post>>, Status> {
    let result = query_as::<_, Post>("SELECT * FROM posts")
        .fetch_all(pool.inner())
        .await;

    match result {
        Ok(posts) => Ok(Json(posts)),
        Err(_) => Err(Status::InternalServerError),
    }
}

// Function to get a single post by id from the database
#[rocket::get("/posts/<post_id>")]
pub async fn get_post(pool: &rocket::State<MySqlPool>, post_id: i32) -> Result<Json<Post>, Status> {
    let result = query_as::<_, Post>("SELECT * FROM posts WHERE id = ?")
        .bind(post_id)
        .fetch_optional(pool.inner())
        .await;

    match result {
        Ok(Some(post)) => Ok(Json(post)),
        Ok(None) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

// Function to create a post in the database
#[rocket::post("/posts", format = "json", data = "<post>")]
pub async fn create_post(pool: &rocket::State<MySqlPool>, post: Json<Post>) -> Result<Status, Status> {
    let insert_result = query("INSERT INTO posts (title, content) VALUES (?, ?)")
        .bind(&post.title)
        .bind(&post.content)
        .execute(pool.inner())
        .await;

    match insert_result {
        Ok(_) => Ok(Status::Created),
        Err(err) => {
            eprintln!("Error inserting post into the database: {:?}", err);
            Err(Status::InternalServerError)
        }
    }
}
