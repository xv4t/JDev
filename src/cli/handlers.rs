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
            // what join_all eo
            // 1. `join_all` wraps all futures into one combined future
            // 2. `.await` hands that combined future to the **tokio runtime**
            // 3. The runtime **polls all futures concurrently** within the current task
            // 4. Each future runs until it hits its own `.await` (like a DB call or HTTP request), then **yields** back
            // 5. The runtime keeps polling whichever futures are ready
            // 6. Once **all** futures complete, `join_all` collects all results into a `Vec` and returns it
            let in_db = try_join_all(lower_case_tags.iter().map(
                |name| queries::check_for_tag_query(name, pool.clone())
            )).await?.iter();
            let new_tags: Vec<String>=lower_case_tags.iter().filter(|| );

            Ok(())
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