use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Login {
    pub uname: String,
    pub pword: String,
}

#[derive(Clone, Deserialize, Debug)]
enum Role {
    Admin,
    Student,
    Teacher,
    Ipm,
}

#[derive(Clone, Deserialize)]
pub struct NewUser {
    pub uname: String,
    pub pword: String,
    pub utype: String,
}

#[derive(sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub uname: String,
    pub pword: String,
    pub utype: String,
}

impl User {
    pub fn has_role(&self, required_role: &str) -> bool {
        self.utype == required_role
    }
}
