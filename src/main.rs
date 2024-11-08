use actix_web::{web, App, HttpServer, HttpResponse};
use mysql::*;
use mysql::prelude::*;
use serde::{Serialize, Deserialize};
use dotenv::dotenv;
use std::env;

#[derive(Serialize, Deserialize, Clone)]
struct User {
    id: Option<u32>,
    name: String,
    email: String,
}

fn get_mysql_pool() -> Pool {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Pool::new(url).expect("Failed to create MySQL pool")
}

async fn get_users() -> HttpResponse {
    let pool = get_mysql_pool();
    let mut conn = pool.get_conn().expect("Failed to get MySQL connection");

    let users: Vec<User> = conn
        .query_map("SELECT id, name, email FROM users", |(id, name, email)| User { id, name, email })
        .expect("Failed to query users");

    HttpResponse::Ok().json(users)
}

async fn create_user(user: web::Json<User>) -> HttpResponse {
    let pool = get_mysql_pool();
    let mut conn = pool.get_conn().expect("Failed to get MySQL connection");

    conn.exec_drop(
        "INSERT INTO users (name, email) VALUES (:name, :email)",
        params! {
            "name" => &user.name,
            "email" => &user.email,
        },
    ).expect("Failed to insert user");

    HttpResponse::Created().json(user.0.clone())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/users", web::get().to(get_users))
            .route("/users", web::post().to(create_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
