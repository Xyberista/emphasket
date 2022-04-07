use rusqlite::Connection;
use std::fs;
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::term::Term;

const BASE: &str = "./book/src/";

pub fn run() -> rusqlite::Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("./words.sqlite3")?;
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
        .map(|t| t.unwrap())
        .collect();

    setup(&terms)?;
    Ok(())
}

fn setup(terms: &[Term]) -> Result<(), Box<dyn std::error::Error>> {
    create_readme()?;
    create_summary(terms)
}

fn create_readme() -> Result<(), Box<dyn std::error::Error>> {
    fs::write(
        BASE.to_string() + "README.md",
        "# Terms\n\nThis project contains 120 statistical terms.\n",
    )?;
    Ok(())
}

fn create_summary(terms: &[Term]) -> Result<(), Box<dyn std::error::Error>> {
    let filepath = BASE.to_owned() + "SUMMARY.md";
    let summary_file = fs::File::create(&filepath)?;
    let mut writer = BufWriter::new(summary_file);

    writeln!(writer, "# Summary\n")?;
    writeln!(writer, "[Introduction](README.md)\n")?;
    writeln!(writer, "# Terms\n")?;
    for term in terms {
        let line = format!(
            "- [{}]({}{}.md)",
            term.name(),
            "terms/",
            term.filename()
        );
        writeln!(writer, "{line}")?;
    }

    writer.flush()?;
    Ok(())
}
