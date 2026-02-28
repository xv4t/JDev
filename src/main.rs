use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "JDev",
    version = "1.0.0",
    about = "a tool to serve people log their advancements",
    subcommand_required = true,
    arg_required_else_help = true,
)]
struct Args{
    #[command(subcommand)]
    sub: Subs,
}

#[derive(Subcommand)]
enum Subs {
    New{
        #[arg(long, value_delimiter = ',', num_args=1.., required= true)]
        tags: Vec<String>,
        #[arg(short, long)]
        title: String,
        #[arg(short, long)]
        description: String
    },
    Del{
        #[arg(short, long, conflicts_with = "tags")]
        id: Option<u64>,
        #[arg(long, value_delimiter = ',', conflicts_with = "id", num_args=1..)]
        tags: Option<Vec<String>>,
    }, 
    Update{
        #[arg(short, long)]
        id: u64,
        #[arg(long, value_delimiter= ',', num_args=0..)]
        tags: Option<Vec<String>>, 
        #[arg(short, long)]
        title: Option<String>,
        #[arg(short, long)]
        description: Option<String>,
    },
    // History{

    // },
}

fn main() {

    let cli = Args::parse();

    match cli.sub {
        Subs::New { title, description, tags } => {
            println!("=== NEW ===");
            println!("  title      : {title}");
            println!("  description: {description}");
            println!("  tags       : {}", tags.join(", "));
        }

        Subs::Del { id, tags } => {
            println!("=== DEL ===");
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
        }

        Subs::Update { id, title, description, tags } => {
            println!("=== UPDATE ===");
            println!("  id: {id}");
            if let Some(t) = title       { println!("  new title      : {t}"); }
            if let Some(d) = description { println!("  new description: {d}"); }
            if let Some(ts) = tags       { println!("  new tags       : {}", ts.join(", ")); }
        }
    }
}
