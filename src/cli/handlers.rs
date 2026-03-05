use super::commands::{Args, Subs};
use super::super::db::queries;
use std::sync::Arc;
use sqlx::{Pool, Postgres};
use futures::future::try_join_all;

pub async fn handle(cmd: Args, pool: Arc<Pool<Postgres>>)-> Result<(), sqlx::Error>{
    match cmd.get_sub(){

        Subs::New { tags, title, description }=> {
            //self explanatory..
            let lower_case_tags: Vec<String>=tags.iter().map(|t| t.to_lowercase()).collect();
            //checks if a tag exists, if so, it's id gets returned, else it is created and its id gets returned
            let tags_ids = try_join_all(lower_case_tags.iter().map(
                |name| queries::exists_or_create_tag(name, pool.clone())
            )).await?;
            //create the log
            let new_log = queries::new_log_query(title, description, pool.clone())
                .await?;
            //create the relation between a log and its tags;
            let _= try_join_all(tags_ids.iter().map(|tag_id| 
                queries::log_tag_register_relation(new_log, *tag_id, pool.clone()))).await?;
            Ok(())
        },

        Subs::Del { id, tags }=> {
            match (id, tags) {
                //delete with log id 
                (Some(id), _) => {
                    //check if a log with the provided id exists in DB
                    if !queries::check_for_log(*id, pool.clone()).await? {
                        eprintln!("Error: no log with the provided id");
                        std::process::exit(1);
                    }
                    //confirming deletion
                    println!("confirm deletion? [Y/N] :");
                    let mut confirmation = String::new();
                    'outer: loop {
                        std::io::stdin().read_line(&mut confirmation).expect("Failed to read line");
                        let confirmation = confirmation.trim().chars().next();
                        match confirmation{
                            Some(c) => {
                                if c.to_ascii_lowercase()=='y' {break 'outer;}
                                else if c.to_ascii_lowercase()=='n' {
                                    println!("deleting canceled");
                                    return Ok(());
                                } 
                                else {println!("invalid input. please enter 'y' for yes or 'n' for no.");}
                            },
                            None => {println!("no character entered. please enter 'y' for yes or 'n' for no.");}
                        }
                    }
                    //soft delete the log
                    queries::delete_log(*id, pool.clone()).await?;
                }
                //delete all logs with a tag/multiple tags
                (_, Some(tags)) => {
                    println!("  deleting items matching tags: {}", tags.join(", "));
                }
                //error if neither were specified
                (None, None) => {
                    eprintln!("Error: provide either --id or --tags");
                    std::process::exit(1);
                }
            }
            Ok(())
        },

        Subs::Update { id, tags, title, description}=> {
            println!("  id: {id}");
            if let Some(t) = title       { println!("  new title      : {t}"); }
            if let Some(d) = description { println!("  new description: {d}"); }
            if let Some(ts) = tags       { println!("  new tags       : {}", ts.join(", ")); }
            Ok(())
        },

        Subs::Get {}=> {
            println!("getting your logs from newest to oldest");
            Ok(())
        }
    }
}