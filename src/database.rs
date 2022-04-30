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
            user_definition         TEXT,
            example                 TEXT,
            picture_name            TEXT
        )",
        [],
    )?;
    Ok(conn)
}

pub fn insert_term(conn: &Connection, term: &Term) -> Result<()> {
    conn.execute(
        "INSERT INTO words (id, term, book_definition, user_definition, example) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            term.id,
            term.term,
            term.book_definition,
            term.user_definition,
            term.example,
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
                example: row.get(4)?,
                picture_link: row.get(5)?,
            })
        })?
        .map(std::result::Result::unwrap)
        .collect();
    Ok(terms)
}

pub fn recount() -> Result<()> {
    let conn = connect()?;
    let terms = get_terms(&conn)?
        .into_iter()
        .enumerate()
        .map(|(i, s)| Term { id: i + 1, ..s });
    for term in terms {
        conn.execute(
            "UPDATE words SET id = $1 WHERE term = $2",
            params![term.id, term.term],
        )?;
    }
    Ok(())
}
