// author: xunfei
// Project: untitled4
// File: pool
// Date: 2023/12/24 14:33

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct PostgresqlPoll<'a> {
    pub max_connections: u32,
    pub url: &'a str,
}

impl PostgresqlPoll<'_> {
    pub fn new(max_connections: u32, url: &str) -> PostgresqlPoll {
        PostgresqlPoll {
            max_connections,
            url,
        }
    }
}
pub async fn get_pool(pool: PostgresqlPoll<'_>) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(pool.max_connections)
        .connect(pool.url)
        .await?;
    Ok(pool)
}
