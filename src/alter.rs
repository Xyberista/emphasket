use rusqlite::params;

use crate::database::{connect, get_terms};

pub fn alter() -> rusqlite::Result<()> {
    let conn = connect()?;
    let terms = get_terms(&conn)?;
    for term in terms {
        conn.execute(
            "UPDATE words SET term = ($1), book_definition = ($2) WHERE id = ($3)",
            params![term.term.trim(), term.book_definition.trim(), term.id],
        )?;
    }
    Ok(())
}
