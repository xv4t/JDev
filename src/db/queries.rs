use sqlx::{Pool, Postgres, FromRow};
use std:: sync::Arc;

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

#[derive(FromRow)]
pub struct Log{
    id: i32,
    title: String,
    content: String
}

impl Log {
    pub fn set_title(&mut self, t: String)  {self.title = t;}
    pub fn set_description(&mut self, c: String)  {self.content = c;}
    pub fn get_id(&self)->i32    {return self.id;}
    pub fn get_title(&self)->&str {return &self.title;}
    pub fn get_content(&self)->&str {return &self.content;}
}

pub async fn get_log_by_id(id: i32, pool: Arc<Pool<Postgres>>)-> Result<Option<Log>, sqlx::Error>{
    let found:Option<Log> = sqlx::query_as(
        "SELECT id, title, content FROM logs WHERE id = $1 ")
        .bind(id)
        .fetch_optional(&*pool).await?;
    
    Ok(found)
}

pub async fn update_log(log: Log, pool: Arc<Pool<Postgres>>)-> Result<(), sqlx::Error>{
    sqlx::query("UPDATE logs SET title = $1 , content = $2 WHERE id = $3;")
    .bind(log.get_title())
    .bind(log.get_content())
    .bind(log.get_id())
    .execute(&*pool).await?;

    Ok(())
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

pub async fn log_tag_delete_realtion(log_id: i32, tag_id: i32, pool: Arc<Pool<Postgres>>)-> Result<(), sqlx::Error>{
    sqlx::query("DELETE FROM logs_tags WHERE log_id = $1 AND tag_id = $2;")
    .bind(log_id).bind(tag_id)
    .execute(&*pool).await?;

    Ok(())
}

pub async fn get_logs_by_tags(tags_ids: Vec<i32>, pool: Arc<Pool<Postgres>>)-> Result<Vec<(i32,)>, sqlx::Error>{
    let result: Vec<(i32,)> = sqlx::query_as(
        "SELECT id FROM logs WHERE EXISTS(
            SELECT 1 FROM logs_tags WHERE logs_tags.log_id = logs.id AND logs_tags.tag_id = ANY($1)    
        );"
    )
    .bind(tags_ids.as_slice())
    .fetch_all(&*pool).await?;
    Ok(result)
}

pub async fn get_tags_by_log(id: i32, pool: Arc<Pool<Postgres>>)-> Result<Vec<(i32,)>, sqlx::Error>{
    let found: Vec<(i32,)> = sqlx::query_as(
        "SELECT tag_id FROM logs_tags WHERE log_id = $1;")
        .bind(id)
        .fetch_all(&*pool).await?;

    Ok(found)
}