use rusqlite::{Connection, Result};
use std::io::{self, Write};

use crate::{database::insert_term, term::Term};

const PROMPT: &str = "λ>";

fn prompt(prompt_for: &str) {
    println!("{prompt_for}");
    print!("{PROMPT}");
    io::stdout().flush().unwrap();
}

pub fn run(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT * from words")?;
    let count = stmt.query_map([], |_| Ok(()))?.count();
    let id = count + 1;

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
                add_single_term(conn, id);
            }
            "2" => {
                add_multiple_terms(conn, id);
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
fn add_single_term(conn: &Connection, id: usize) -> bool {
    prompt("Term: (Leave empty to abort)");
    let mut term = String::new();
    io::stdin()
        .read_line(&mut term)
        .expect("Failed to read line");

    if term.is_empty() {
        return false;
    }
    let term = term.trim().to_lowercase();

    prompt("Book definition:");
    let mut book_definition = String::new();
    io::stdin()
        .read_line(&mut book_definition)
        .expect("Failed to read line");
    let book_definition = book_definition.trim().to_string();

    // TODO: implement another command without this portion for easier data entry
    let user_definition = String::new();
    // prompt("User definition:");
    // let mut user_definition = String::new();
    // io::stdin()
    //     .read_line(&mut user_definition)
    //     .expect("Failed to read line");
    let user_definition = user_definition.trim().to_string();

    let term = Term {
        id,
        term,
        book_definition,
        user_definition,
    };

    if let Err(e) = insert_term(conn, &term) {
        println!();
        println!("Error: {}", e);
    }
    println!();
    true
}

/// Adds terms continuously until an empty term is inputted
fn add_multiple_terms(conn: &Connection, id: usize) {
    let mut id = id;
    loop {
        let added = add_single_term(conn, id);
        if !added {
            break;
        }
        id += 1;
    }
}
