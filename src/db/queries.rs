use sqlx::{Pool, Postgres};
use std::sync::Arc;

pub async fn new_log_query(title: &str, description: &str, pool: Arc<Pool<Postgres>>)
-> Result<i32, sqlx::Error>{
    let result: (i32,)= sqlx::query_as("INSERT INTO logs (title, content) VALUES ($1, $2) RETURNING id")
    .bind(title)
    .bind(description)
    .fetch_one(&*pool).await?;

    Ok(result.0)
}

pub async fn check_for_tag_query(tag: &str, pool: Arc<Pool<Postgres>>)-> Result<bool, sqlx::Error>{
    let found: bool = sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM tags WHERE name = ?;")
    .bind(tag).fetch_one(&*pool).await?;

    Ok(found)
}

pub async fn new_tag_query(tag: &str, pool:Arc<Pool<Postgres>>)-> Result<i32, sqlx::Error>{
    let result: (i32,)=sqlx::query_as("INSERT INTO tags (name) VALUES ($1) RETURNING id")
    .bind(tag).fetch_one(&*pool).await?;

    Ok(result.0)
}