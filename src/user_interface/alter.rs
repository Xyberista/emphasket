use rusqlite::{params, Connection};
use std::io;

use super::{multiline_input_with, prompt, prompt_with};
use crate::term::Term;

pub fn alter_entry(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    prompt_with("Which term would you like to alter? (Case insensitive)");
    let mut term = String::new();
    io::stdin()
        .read_line(&mut term)
        .expect("Failed to read line");
    let term: String = term.trim().to_lowercase();
    let mut term: Term =
        conn.query_row("SELECT * FROM words WHERE term = ($1)", [term], |row| {
            Ok(Term {
                id: row.get(0)?,
                term: row.get(1)?,
                book_definition: row.get(2)?,
                user_definition: row.get(3)?,
                example: row.get(4)?,
                picture_link: row.get(5)?,
            })
        })?;

    println!("{term}\n");
    println!("What would you like to alter?");
    println!("1: Term");
    println!("2: Book Definition");
    println!("3: User Definition");
    println!("4: Example");
    println!("F: Finish Editing");
    println!();

    loop {
        prompt();
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        println!();
        match choice.trim() {
            "1" => alter_term(&mut term),
            "2" => alter_book_definition(&mut term),
            "3" => alter_user_definition(&mut term),
            "4" => alter_example(&mut term),
            "F" => break,
            _ => {}
        }
        println!("\n{term}");
    }

    conn.execute(
        "UPDATE words
        SET term = $1,
            book_definition = $2,
            user_definition = $3,
            example = $4
        WHERE id = $5",
        params![
            term.term,
            term.book_definition,
            term.user_definition,
            term.example,
            term.id
        ],
    )?;

    Ok(())
}

fn alter_term(term: &mut Term) {
    prompt_with("What is the new term?");
    let mut new_term = String::new();
    io::stdin()
        .read_line(&mut new_term)
        .expect("Failed to read line");
    term.term = new_term;
}

fn alter_book_definition(term: &mut Term) {
    term.book_definition = multiline_input_with("New Book Definition");
}
fn alter_user_definition(term: &mut Term) {
    prompt_with("What is the new user definition?");
    let mut new_user_definition = String::new();
    io::stdin()
        .read_line(&mut new_user_definition)
        .expect("Failed to read line");
    term.user_definition = new_user_definition;
}
fn alter_example(term: &mut Term) {
    term.example = multiline_input_with("New example");
}
