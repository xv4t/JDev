use super::commands::{Args, Subs};
use super::super::db::queries;
use core::fmt;
use std::sync::Arc;
use sqlx::{Pool, Postgres};
use futures::future::try_join_all;
use std::io::{self, Write};

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
                    if !(queries::check_for_log(*id, pool.clone()).await?) {
                        eprintln!("Error: no log with id {id}");
                        std::process::exit(1);
                    }
                    //confirming deletion
                    print!("confirm deletion? [Y/N] :");
                    io::stdout().flush().unwrap();
                    let mut counter =1;
                    'outer: loop{
                        let answer = confirm();
                        println!("{answer}");
                        match answer{
                            Confirmation::Confirmed => break 'outer,
                            Confirmation::Canceled => return Ok(()),
                            Confirmation::Invalid => {
                                if counter == 3 {return Ok(());}
                                counter+=1
                            }
                        }
                    }   
                    //soft delete the log
                    queries::delete_log(*id, pool.clone()).await?;
                }
                //delete all logs with a tag/multiple tags
                (_, Some(tags)) => {
                    println!("deleting items matching tags: {} , non existing tags will be ignored", tags.join(", "));
                    let lower_case_tags: Vec<String>=tags.iter().map(|t| t.to_lowercase()).collect();
                    //get IDs of existing tags and None for non-existing ones
                    let tags_check_result = try_join_all(lower_case_tags.iter().map(|tag|
                        queries::check_for_tag_query(tag, pool.clone()))).await?;
                    if tags_check_result.is_empty(){
                        eprintln!("no valid tags were provided");
                        return Ok(());
                    }
                    //confirm deletion
                    print!("confirm deletion? [Y/N] :");
                    io::stdout().flush().unwrap();
                    let mut counter =1;
                    'outer: loop{
                        let answer = confirm();
                        println!("{answer}");
                        match answer{
                            Confirmation::Confirmed => break 'outer,
                            Confirmation::Canceled => return Ok(()),
                            Confirmation::Invalid => {
                                if counter == 3 {return Ok(());}
                                counter+=1
                            }
                        }
                    } 
                    //filter out non-existing tags
                    let existing_tags_ids: Vec<i32> = tags_check_result.iter().map(
                        |result| 
                        match result{
                            Some(id) => (id.0, true),
                            None => (0, false)
                        }
                    ).filter(|(_,found)|*found)
                    .map(|(id, _)| id).collect();
                    //get logs with any of those tags
                    let logs = queries::get_logs_by_tags(existing_tags_ids, pool.clone()).await?;
                    let _=try_join_all(logs.iter().map(|log|
                        queries::delete_log(log.0, pool.clone()))).await?;
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
            //check if a log with the id exists in DB and if anything was to edit
            let mut log = match queries::get_log_by_id(*id, pool.clone()).await?{
                Some(result) => result,
                None =>{
                    eprintln!("no log with id {id} exists");
                    std::process::exit(1);
                }
            };
            if let(None, None, None) = (tags, title, description) {
                eprintln!("nothing to edit");
                return Ok(());
            }
            //confirm updating
            print!("confirm upadte? [Y/N] :");
            io::stdout().flush().unwrap();
            let mut counter =1;
            'outer: loop{
                let answer = confirm();
                println!("{answer}");
                match answer{
                    Confirmation::Confirmed => break 'outer,
                    Confirmation::Canceled => return Ok(()),
                    Confirmation::Invalid => {
                        if counter == 3 {return Ok(());}
                        counter+=1
                    }
                }
            }
            //edit title and content if provided 
            if let Some(t) = title {log.set_title(t.to_string());}
            if let Some(d) = description {log.set_description(d.to_string());}
            queries::update_log(log, pool.clone()).await?;
            //if no tags were provided the task ends
            if let None = tags {return Ok(());}
            //create and get new tags IDs
            let tags = tags.as_ref().unwrap();
            let lower_case_tags: Vec<String>=tags.iter().map(|t| t.to_lowercase()).collect();        
            let new_tags_ids = try_join_all(lower_case_tags.iter().map(
                |name| queries::exists_or_create_tag(name, pool.clone())
            )).await?;
            //get origianl log tags
            let original_tags_ids :Vec<i32>= queries::get_tags_by_log(*id, pool.clone()).await?
                .iter().map(|i| i.0).collect();
            //figure which tags to add to the log and which to remove
            let mut to_add: Vec<i32> = Vec::new();
            for i in new_tags_ids.iter()  {if !original_tags_ids.contains(i) {to_add.push(*i);} }
            let mut to_delete: Vec<i32> = Vec::new();
            for i in original_tags_ids  {if !new_tags_ids.contains(&i) {to_delete.push(i);} }

            try_join_all(to_delete.iter().map(|i: &i32| 
                queries::log_tag_delete_realtion(*id, *i, pool.clone()))).await?;
            try_join_all(to_add.iter().map(|i|
                queries::log_tag_register_relation(*id, *i, pool.clone()))).await?;
            Ok(())
        },

        Subs::Get {}=> {
            println!("getting your logs from newest to oldest");
            Ok(())
        }
    }
}

#[non_exhaustive]
enum Confirmation{
    Confirmed,
    Canceled,
    Invalid,
}

impl fmt::Display for Confirmation{
    fn fmt(&self, f: &mut fmt::Formatter)-> fmt::Result{
        match self{
            Confirmation::Confirmed => write!(f, "Confirmed"),
            Confirmation::Canceled => write!(f, "Canceled"),
            Confirmation::Invalid => write!(f, "invalid input. please enter 'y' for yes or 'n' for no."),
        }
    }
}

fn confirm()-> Confirmation{
    let mut confirmation = String::new();
    loop {
        std::io::stdin().read_line(&mut confirmation).expect("Failed to read line");
        let confirmation = confirmation.trim().chars().next();
        match confirmation{
            Some(c) => {
                if c.to_ascii_lowercase()=='y' {return Confirmation::Confirmed}
                else if c.to_ascii_lowercase()=='n' {return Confirmation::Canceled;} 
                else {return Confirmation::Invalid;}
            },
            None => {return Confirmation::Invalid;}
        }
    }
}
