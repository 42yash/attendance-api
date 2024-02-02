mod db;
mod models;
mod routes;

#[macro_use]
extern crate rocket;

use routes::{index, login, register};
use sqlx::postgres::PgPool;
use std::env;

#[launch]
async fn rocket() -> _ {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create DB pool");

    // Call the function to create the users table
    db::create_users_table(&db_pool)
        .await
        .expect("Failed to create users table");

    rocket::build()
        .manage(db_pool)
        .mount("/", routes![index, register, login])
}
