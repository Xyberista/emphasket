use rusqlite::params;

use crate::{database::connect, term::Term};

pub fn alter() {
    let conn = connect().unwrap();
    let mut stmt = conn.prepare("SELECT * FROM words").unwrap();
    let terms: Vec<Term> = stmt
        .query_map([], |row| {
            Ok(Term {
                id: row.get(0).unwrap(),
                term: row.get(1).unwrap(),
                book_definition: row.get(2).unwrap(),
                user_definition: row.get(3).unwrap(),
            })
        })
        .unwrap()
        .map(|t| t.unwrap())
        .collect();
    for term in terms {
        conn.execute(
            "UPDATE words SET term = ($1), book_definition = ($2) WHERE id = ($3)",
            params![term.term.trim(), term.book_definition.trim(), term.id],
        )
        .unwrap();
    }
}
