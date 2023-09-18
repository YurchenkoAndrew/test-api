mod services;

use std::io::Result;
use actix_web::{HttpServer, App, web::Data};
use dotenv::dotenv;
use services::{fetch_users, get_user_posts, create_post};
use sqlx::{Postgres, Pool, postgres::PgPoolOptions};

pub struct AppState {
    db: Pool<Postgres>
}

#[actix_web::main]
async fn main() -> Result<()>{
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error binding a connection pool!");
    sqlx::migrate!("./migrations").run(&pool).await.expect("Migrations failed!");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState{ db: pool.clone() }))
            .service(fetch_users)
            .service(get_user_posts)
            .service(create_post)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}