use clap::Parser;
use cli::{Cli, Commands};

mod book;
mod cli;
pub mod database;
mod download;
mod term;
mod user_interface;
mod util;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Some(command) = Cli::parse().command {
        match command {
            Commands::Book => book::run()?,
            Commands::Recount => database::recount()?,
            Commands::Download => download::download()?,
            Commands::Util => util::run()?,
        }
    } else {
        let conn = database::connect()?;
        user_interface::run(&conn)?;
    }
    Ok(())
}
