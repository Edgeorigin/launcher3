pub mod v1;
pub mod shared;
pub mod app;

// use clap::{Arg, Parser, Subcommand};

// #[derive(Parser)]
// #[command(author, version, about, long_about = None)]
// #[command(propagate_version = true)]
// struct Cli {
//     #[command(subcommand)]
//     command: Commands,
// }

// #[derive(Subcommand)]
// enum Commands {
//     /// Adds files to myapp
//     Add(Add),
// }

// #[derive(Args)]
// struct Add {
//     name: Option<String>,
// }

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    Ok(())
}