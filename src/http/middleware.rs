// src/middleware.rs

use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};

use crate::db::models::User;

#[derive(Debug)]
pub struct Admin;

#[derive(Debug)]
pub struct Teacher;

#[derive(Debug)]
pub struct Student;

#[derive(Debug)]
pub struct IPM;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Teacher {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let user = req.rocket().state::<User>();
        match user {
            Some(user) if user.utype == "Teacher" => Outcome::Success(Teacher),
            _ => Outcome::Error((Status::Forbidden, ())),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Student {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let user = req.rocket().state::<User>();
        match user {
            Some(user) if user.utype == "Student" => Outcome::Success(Student),
            _ => Outcome::Error((Status::Forbidden, ())),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for IPM {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let user = req.rocket().state::<User>();
        match user {
            Some(user) if user.utype == "IPM" => Outcome::Success(IPM),
            _ => Outcome::Error((Status::Forbidden, ())),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Admin {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let user = req.rocket().state::<User>();
        match user {
            Some(user) if user.has_role("admin") => Outcome::Success(Admin),
            _ => Outcome::Error((Status::Forbidden, ())),
        }
    }
}
