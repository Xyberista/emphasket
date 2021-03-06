use rusqlite::{Connection, Result};
use std::io::{self, Write};

use crate::{database::insert_term, term::Term};

mod alter;

const PROMPT: &str = "λ>";

fn prompt() {
    print!("{PROMPT}");
    io::stdout().flush().unwrap();
}

fn prompt_with(prompt_for: &str) {
    println!("{prompt_for}");
    prompt();
}

pub fn run(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT * from words")?;
    let count = stmt.query_map([], |_| Ok(()))?.count();
    let id = count + 1;

    loop {
        println!("Statistics Project Menu Screen");
        println!("1: Add Single Term");
        println!("2: Add Multiple Terms");
        println!("3: Alter Term");
        println!("Q: Quit");
        println!();

        prompt_with("command");

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
            "3" => {
                alter::alter_entry(conn).unwrap();
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
    prompt_with("Term: (Leave empty to abort)");
    let mut term = String::new();
    io::stdin()
        .read_line(&mut term)
        .expect("Failed to read line");

    if term.is_empty() {
        return false;
    }
    let term = term.trim().to_lowercase();

    let book_definition = multiline_input_with("Book definition");

    // TODO: implement another command without this portion for easier data entry
    let user_definition = String::new();
    // prompt("User definition:");
    // let mut user_definition = String::new();
    // io::stdin()
    //     .read_line(&mut user_definition)
    //     .expect("Failed to read line");
    let user_definition = user_definition.trim().to_string();

    // TODO: Add example input somehow
    let example = String::new();

    let term = Term {
        id,
        term,
        book_definition,
        user_definition,
        example,
        picture_link: String::new(),
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

fn multiline_input_with(prompt_for: &str) -> String {
    let mut result = String::new();
    println!("{}: (Empty line to finish)", prompt_for);
    loop {
        prompt();
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let line = line.trim_end();
        if line.is_empty() {
            break;
        }
        result += line;
        result += "\n";
    }
    result
}
