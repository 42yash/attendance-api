use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest};
use rocket::Request;
use serde::{Deserialize, Serialize};

// JWT secret key
// NOTE: In production, you should use a more secure way of handling the secret key,
// such as environment variables or a configuration file.
const SECRET_KEY: &[u8] = b"your_secret_key";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,   // Subject (Username)
    exp: usize,    // Expiration time
    user_id: i32,  // User ID from the User struct
    utype: String, // User type (role) from the User struct
}

pub fn create_token(
    user_id: i32,
    uname: &str,
    utype: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: uname.to_owned(),
        exp: expiration,
        user_id,
        utype: utype.to_owned(),
    };

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(SECRET_KEY),
    )
}

pub fn decode_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY),
        &Validation::new(Algorithm::HS256),
    )
    .map(|data| data.claims)
}

pub struct AuthenticatedUser {
    pub user_id: i32,
    pub uname: String,
    pub utype: String,
}

#[derive(Debug)]
pub enum AuthError {
    BadRequest,
    Unauthorized,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = AuthError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Error((Status::BadRequest, AuthError::BadRequest));
        }

        let token = keys[0].trim_start_matches("Bearer ").to_string();
        match decode_token(&token) {
            Ok(claims) if Utc::now().timestamp() as usize <= claims.exp => {
                Outcome::Success(AuthenticatedUser {
                    user_id: claims.user_id,
                    uname: claims.sub,
                    utype: claims.utype,
                })
            }
            _ => Outcome::Error((Status::Unauthorized, AuthError::Unauthorized)),
        }
    }
}
