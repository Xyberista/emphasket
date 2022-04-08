use alter::alter;
use clap::Parser;
use cli::{Cli, Commands};

mod alter;
mod book;
mod cli;
pub mod database;
mod term;
mod user_interface;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Some(command) = Cli::parse().command {
        match command {
            Commands::Book => book::run()?,
            Commands::Alter => alter()?,
        }
    } else {
        let conn = database::connect()?;
        user_interface::run(&conn)?;
    }
    Ok(())
}
