use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Copies files to the book directory
    Book,
    /// Recount the database rows
    Recount,
    /// Downloads the pictures for each example in the database
    Download,
    /// Get approximate word count
    Util,
}
