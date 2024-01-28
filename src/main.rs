mod commands;
mod git;
mod utils;

use clap::{command, Parser};
use clap::{Args, Subcommand};

use commands::{
    handle_add, handle_clone, handle_init, handle_link, handle_remove, handle_status, handle_sync,
};

#[derive(Debug, Parser)]
#[command(name = "dottler")]
#[command(author, version)]
#[command(
    about = "Dottle your dotfiles",
    long_about = "Dottle your dotfiles. Dottler is your friendly dotfile manager and dottle keeper. Set up dottles to store different versions of your dotfiles."
)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(name = "init")]
    Init,

    #[command(name = "link")]
    Link(LinkArgs),

    #[command(name = "clone")]
    Clone(CloneArgs),

    #[command(name = "add")]
    Add(AddArgs),

    #[command(name = "remove")]
    Remove(RemoveArgs),

    #[command(name = "sync")]
    Sync,

    #[command(name = "status")]
    Status,
}

#[derive(Debug, Args)]
pub struct LinkArgs {
    pub url: String,
}

#[derive(Debug, Args)]
pub struct CloneArgs {
    pub url: String,
}

#[derive(Debug, Args)]
pub struct AddArgs {
    files: Vec<String>,
}

#[derive(Debug, Args)]
pub struct RemoveArgs {
    file: String,
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Init => handle_init(),
        Commands::Link(args) => handle_link(args),
        Commands::Clone(args) => handle_clone(args),
        Commands::Add(args) => handle_add(args),
        Commands::Remove(args) => handle_remove(args),
        Commands::Sync => handle_sync(),
        Commands::Status => handle_status(),
    }
}
