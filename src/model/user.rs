use crate::common::{
    pool::{get_pool, PostgresqlPoll},
    variable::{MAX_CONNECTIONS, URL},
};
use actix_web::web::service;
use serde::{Deserialize, Serialize};
use sqlx::Executor;
#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct User {
    pub(crate) id: i32,
    username: Option<String>,
    nickname: Option<String>,
    password: Option<String>,
    avatar: Option<String>,
}

pub async fn retrieve_user(user_id: i32) -> Result<User, sqlx::Error> {
    let pool = get_pool(PostgresqlPoll::new(MAX_CONNECTIONS, URL)).await?;
    let user: User = sqlx::query_as("Select * from account where id = $1")
        .bind(user_id)
        .fetch_one(&pool)
        .await?;
    Ok(user)
}

pub async fn list_users(limit: i32, offset: i32) -> Result<Vec<User>, sqlx::Error> {
    let pool = get_pool(PostgresqlPoll::new(MAX_CONNECTIONS, URL)).await?;
    let users: Vec<User> = sqlx::query_as("Select * from account limit $1 offset $2")
        .bind(limit)
        .bind(offset)
        .fetch_all(&pool)
        .await?;
    Ok(users)
}

pub async fn update_user(user: User) -> Result<(), sqlx::Error> {
    let pool = get_pool(PostgresqlPoll::new(MAX_CONNECTIONS, URL)).await?;
    sqlx::query("update account set username = $1 where id = $2")
        .bind(user.username)
        .bind(user.id)
        .execute(&pool)
        .await?;
    Ok(())
}

pub async fn delete_user(user_id: i32) -> Result<(), sqlx::Error> {
    let pool = get_pool(PostgresqlPoll::new(MAX_CONNECTIONS, URL)).await?;
    sqlx::query("Delete from account where id = $1")
        .bind(user_id)
        .execute(&pool)
        .await?;
    Ok(())
}
