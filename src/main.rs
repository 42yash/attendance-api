use rocket::serde::json::Json;

mod models;

use models::{Login, Register};

#[macro_use]
extern crate rocket;

#[post("/register", format = "json", data = "<user>")]
fn register(user: Json<Register>) -> String {
    format!("{}! Hello, {}! {}", user.utype, user.uname, user.pword)
}

#[post("/login", format = "json", data = "<user>")]
fn login(user: Json<Login>) -> String {
    format!("Login! Hello, {}! {}", user.uname, user.pword)
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, login, register])
}
