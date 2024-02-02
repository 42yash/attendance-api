use sqlx::postgres::PgPool;
use sqlx::Result;

use crate::models::*;

pub async fn create_users_table(pool: &PgPool) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS Users (
            id SERIAL PRIMARY KEY,
            uname VARCHAR(50) NOT NULL,
            pword VARCHAR(50) NOT NULL,
            utype VARCHAR(50) NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn create_user(pool: &PgPool, user: &NewUser) -> Result<u64> {
    let result = sqlx::query!(
        r#"
        INSERT INTO Users (uname, pword, utype)
        VALUES ($1, $2, $3)
        "#,
        user.uname,
        user.pword,
        user.utype
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

pub async fn get_user_by_username(
    pool: &PgPool,
    username: &str,
) -> Result<Option<User>, sqlx::Error> {
    let row: Option<User> = sqlx::query_as!(
        User,
        r#"
        SELECT * FROM Users WHERE uname = $1
        "#,
        username
    )
    .fetch_optional(pool)
    .await?;

    Ok(row)
}
