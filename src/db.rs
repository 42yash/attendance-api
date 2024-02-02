use sqlx::postgres::PgPool;
use sqlx::Result;

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
