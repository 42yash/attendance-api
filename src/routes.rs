use rocket::serde::json::{json, Json, Value};
use rocket::State;
use sqlx::postgres::PgPool; // Import the middleware module
                            // Import the Admin struct

use crate::{auth, db, middleware::*, models::*};

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

    if let Some(db_user) = user_from_db {
        if db_user.pword == user.pword {
            let token = auth::create_token(db_user.id, &db_user.uname, &db_user.utype)
                .map_err(|_| "Failed to generate token".to_string())?;
            let response = json!({
                "success": true,
                "message": format!("Successfully logged in user {}", user.uname),
                "token": token
            });
            Ok(Json(response))
        } else {
            let response = json!({
                "success": false,
                "message": "Invalid username or password"
            });
            Ok(Json(response))
        }
    } else {
        let response = json!({
            "success": false,
            "message": "Invalid username or password"
        });
        Ok(Json(response))
    }
}

#[get("/admin/dashboard")]
pub fn admin_dashboard(_admin: Admin) -> &'static str {
    "Admin Dashboard"
}

#[get("/teacher/dashboard")]
pub fn teacher_dashboard(_teacher: Teacher) -> &'static str {
    "Teacher Dashboard"
}

#[get("/student/dashboard")]
pub fn student_dashboard(_student: Student) -> &'static str {
    "Student Dashboard"
}

#[get("/ipm/dashboard")]
pub fn ipm_dashboard(_ipm: IPM) -> &'static str {
    "IPM Dashboard"
}

#[get("/")]
pub async fn index() -> String {
    format!("Hello, world!")
}
