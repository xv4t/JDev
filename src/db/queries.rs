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

pub async fn check_for_log(id: i32, pool:Arc<Pool<Postgres>>)-> Result<bool, sqlx::Error>{
    let found: bool = sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM logs WHERE id = $1 AND is_deleted != TRUE)")
    .bind(id)
    .fetch_one(&*pool).await?;

    Ok(found)
}

pub async fn delete_log(id: i32, pool: Arc<Pool<Postgres>>)-> Result<(), sqlx::Error>{
    sqlx::query("UPDATE logs SET is_deleted = TRUE WHERE id = $1;")
    .bind(id)
    .execute(&*pool).await?;

    Ok(())
}

pub async fn check_for_tag_query(tag: &str, pool: Arc<Pool<Postgres>>)-> Result<Option<(i32,)>, sqlx::Error>{
    let found: Option<(i32,)> = sqlx::query_as("SELECT id FROM tags WHERE name = $1;")
    .bind(tag).fetch_optional(&*pool).await?;

    Ok(found)
}

pub async fn new_tag_query(tag: &str, pool:Arc<Pool<Postgres>>)-> Result<i32, sqlx::Error>{
    let result: (i32,)=sqlx::query_as("INSERT INTO tags (name) VALUES ($1) RETURNING id")
    .bind(tag).fetch_one(&*pool).await?;

    Ok(result.0)
}

pub async fn exists_or_create_tag(tag: &str, pool: Arc<Pool<Postgres>>)-> Result<i32, sqlx::Error>{
    let result = check_for_tag_query(tag, pool.clone()).await?;
    match result{
        Some(id) => Ok(id.0),
        None => {
            let new_tag = new_tag_query(tag, pool).await?;
            Ok(new_tag)
        }
    }
}

pub async fn log_tag_register_relation(log_id: i32, tag_id: i32, pool: Arc<Pool<Postgres>>)
-> Result<i32, sqlx::Error>{
    let result: (i32,) = sqlx::query_as(
        "INSERT INTO logs_tags (log_id, tag_id) VALUES ($1, $2) RETURNING id;")
        .bind(log_id).bind(tag_id)
        .fetch_one(&*pool)
        .await?;

    Ok(result.0)
}