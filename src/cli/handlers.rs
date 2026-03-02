#![allow(unused_variables)]

use super::commands::{Args, Subs};
use std::sync::Arc;
use sqlx::{Pool, Postgres};

pub fn handle(cmd: Args, pool: Arc<Pool<Postgres>>){
    match cmd.get_sub(){
        Subs::New { tags, title, description }=> {

        },
        Subs::Del { id, tags }=> {
            match (id, tags) {
                (Some(id), _) => {
                    println!("  deleting item with id = {id}");
                }
                (_, Some(tags)) => {
                    println!("  deleting items matching tags: {}", tags.join(", "));
                }
                (None, None) => {
                    eprintln!("Error: provide either --id or --tags");
                    std::process::exit(1);
                }
            }
        },
        Subs::Update { id, tags, title, description}=> {
            println!("  id: {id}");
            if let Some(t) = title       { println!("  new title      : {t}"); }
            if let Some(d) = description { println!("  new description: {d}"); }
            if let Some(ts) = tags       { println!("  new tags       : {}", ts.join(", ")); }
        },
        Subs::Get {}=> {
            println!("getting your logs from newest to oldest");
        }
    }
}