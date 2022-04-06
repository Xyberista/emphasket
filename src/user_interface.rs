use rusqlite::{Connection, Result};
use std::io::{self, Write};

use crate::{database::insert_term, Term};

const PROMPT: &str = "λ>";

fn prompt(prompt_for: &str) {
    println!("{prompt_for}");
    print!("{PROMPT}");
    io::stdout().flush().unwrap();
}

pub fn run(conn: Connection) -> Result<()> {
    loop {
        println!("Statistics Project Menu Screen");
        println!("1: Add Single Term");
        println!("2: Add Multiple Terms");
        println!("Q: Quit");
        println!();

        prompt("command");

        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");
        println!();

        let command = command.trim();

        match command {
            "1" => {
                add_single_term(&conn)?;
            }
            "2" => {
                add_multiple_terms(&conn)?;
            }
            "Q" | "q" => break,
            _ => continue,
        }
    }
    Ok(())
}

/// Adds a single term to the database
///
/// Aborts the process if the provided term is empty
/// Returns whether a term was added
fn add_single_term(conn: &Connection) -> Result<bool> {
    prompt("Term: (Leave empty to abort)");
    let mut term = String::new();
    io::stdin()
        .read_line(&mut term)
        .expect("Failed to read line");

    if term.is_empty() {
        return Ok(false);
    }

    prompt("Book definition:");
    let mut book_definition = String::new();
    io::stdin()
        .read_line(&mut book_definition)
        .expect("Failed to read line");

    prompt("User definition:");
    let mut user_definition = String::new();
    io::stdin()
        .read_line(&mut user_definition)
        .expect("Failed to read line");

    let term = Term {
        term,
        book_definition,
        user_definition,
    };

    if let Err(e) = insert_term(conn, term) {
        println!();
        println!("Error: {}", e);
    }
    println!();
    Ok(true)
}

/// Adds terms continuously until an empty term is inputted
fn add_multiple_terms(conn: &Connection) -> Result<()> {
    loop {
        let added = add_single_term(conn)?;
        if !added {
            break;
        }
    }
    Ok(())
}
