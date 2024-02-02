use rocket::serde::json::{json, Json, Value};
use rocket::State;
use sqlx::postgres::PgPool;

use crate::{db, models::*};

#[post("/register", format = "json", data = "<user>")]
pub async fn register(user: Json<NewUser>, db_pool: &State<PgPool>) -> Result<Json<Value>, String> {
    let uname = user.uname.clone();
    db::create_user(db_pool.inner(), &user.into_inner())
        .await
        .map_err(|e| e.to_string())?;

    let response = json!({
        "success": true,
        "message": format!("Successfully registered user {}", uname)
    });

    Ok(Json(response))
}

#[post("/login", format = "json", data = "<user>")]
pub async fn login(user: Json<Login>, db_pool: &State<PgPool>) -> Result<Json<Value>, String> {
    let user_from_db = db::get_user_by_username(db_pool.inner(), &user.uname)
        .await
        .map_err(|e| e.to_string())?;

    if user_from_db.is_none() || user_from_db.unwrap().pword != user.pword {
        return Err("Invalid username or password".into());
    }

    let response = json!({
        "success": true,
        "message": format!("Successfully logged in user {}", user.uname)
    });

    Ok(Json(response))
}

#[get("/")]
pub async fn index() -> String {
    format!("Hello, world!")
}
