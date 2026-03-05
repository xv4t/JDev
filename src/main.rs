use jdev::cli::{commands, handlers};
use clap::Parser;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use dotenv::dotenv;
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error>{

    dotenv().ok();
    let database_url=env::var("DATABASE_URL").expect("database must be set");

    let pool:Arc<Pool<Postgres>>=Arc::new(PgPoolOptions::new().connect(&database_url).await?);


    let cli = commands::Args::parse();
    handlers::handle(cli, pool.clone()).await?;

    Ok(())
}
