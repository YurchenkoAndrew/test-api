use actix_web::{Responder, web::{Path, Json, Data}, post, get, HttpResponse};
use serde::{Serialize, Deserialize};
use sqlx::FromRow;

use crate::AppState;

#[derive(Serialize, FromRow)]
struct User {
    id: i32,
    name: String,
    last_name: String
}

#[derive(Serialize, FromRow)]
struct Post {
    id: i32,
    title: String,
    description: String,
    user_id: i32
}

#[derive(Deserialize)]
pub struct CreatePost{
    pub title: String,
    pub description: String
}

#[get("/users")]
pub async fn fetch_users(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, User>("SELECT * FROM users").fetch_all(&state.db).await 
    {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::NotFound().json("Not found users!"),
    }
}

#[get("/users/{id}/posts")]
pub async fn get_user_posts(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let id: i32 = path.into_inner();
    match sqlx::query_as::<_, Post>("SELECT id, title, description, user_id FROM posts WHERE user_id = $1").bind(id).fetch_all(&state.db).await 
    {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(_) => HttpResponse::NotFound().json("Posts not found!"),   
    }
}

#[post("/users/{id}/posts")]
pub async fn create_post(state: Data<AppState>, path: Path<i32>, post: Json<CreatePost>) -> impl Responder {
    let id: i32 = path.into_inner();
    match sqlx::query_as::<_, Post>(
        "INSERT INTO posts (title, description, user_id) VALUES ($1, $2, $3) RETURNING id, title, description, user_id")
                .bind(post.title.to_string()) 
                .bind(post.description.to_string())
                .bind(id)
                .fetch_one(&state.db)
                .await
    {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(_) => HttpResponse::InternalServerError().json("Failed to create post!"),
    }
}