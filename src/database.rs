use rusqlite::{params, Connection, Result};

use crate::term::Term;

/// Sets up a connection to the database, creating a new one if it does not exist.
pub fn connect() -> Result<Connection> {
    let conn = Connection::open("./words.sqlite3")?;

    conn.execute("PRAGMA encoding = 'UTF-16'", [])?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS words (
            id                      INTEGER,
            term                    TEXT PRIMARY KEY,
            book_definition         TEXT,
            user_definition         TEXT
        )",
        [],
    )?;
    Ok(conn)
}

pub fn insert_term(conn: &Connection, term: &Term) -> Result<()> {
    conn.execute(
        "INSERT INTO words (id, term, book_definition, user_definition) VALUES (?1, ?2, ?3, ?4)",
        params![
            term.id,
            term.term,
            term.book_definition,
            term.user_definition
        ],
    )?;
    Ok(())
}

pub fn get_terms(conn: &Connection) -> Result<Vec<Term>> {
    let mut stmt = conn.prepare("SELECT * FROM words")?;
    let terms: Vec<Term> = stmt
        .query_map([], |row| {
            Ok(Term {
                id: row.get(0)?,
                term: row.get(1)?,
                book_definition: row.get(2)?,
                user_definition: row.get(3)?,
            })
        })?
        .map(std::result::Result::unwrap)
        .collect();
    Ok(terms)
}
