use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(
    name = "JDev",
    version = "1.0.0",
    about = "a tool to serve people log their advancements",
    subcommand_required = true,
    arg_required_else_help = true,
)]
pub struct Args{
    #[command(subcommand)]
    sub: Subs,
}

#[derive(Subcommand)]
pub enum Subs {
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
    Get{},
    // History{

    // },
}

impl Args {
    pub fn get_sub(&self)-> &Subs{
        return &self.sub;
    }
}
