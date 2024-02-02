mod db;
mod models;

#[macro_use]
extern crate rocket;

use models::{Login, Register};
use rocket::serde::json::Json;
use rocket::State;
use sqlx::postgres::PgPool;
use std::env;

#[post("/register", format = "json", data = "<user>")]
fn register(user: Json<Register>, db_pool: &State<PgPool>) -> String {
    format!("{}! Hello, {}! {}", user.utype, user.uname, user.pword)
}

#[post("/login", format = "json", data = "<user>")]
fn login(user: Json<Login>, db_pool: &State<PgPool>) -> String {
    format!("Login! Hello, {}! {}", user.uname, user.pword)
}

#[get("/")]
async fn index(db_pool: &State<PgPool>) -> String {
    format!("Hello, world!")
}

#[launch]
async fn rocket() -> _ {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create DB pool");

    db::create_users_table(&db_pool)
        .await
        .expect("Failed to create users table");

    rocket::build()
        .manage(db_pool)
        .mount("/", routes![index, register, login])
}
