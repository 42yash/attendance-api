use serde::Deserialize;

#[derive(Deserialize)]
pub struct Login {
    pub uname: String,
    pub pword: String,
}

#[derive(Deserialize)]
pub struct Register {
    pub uname: String,
    pub pword: String,
    pub utype: String,
}
