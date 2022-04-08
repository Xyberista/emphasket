use std::fs;
use std::io::{BufWriter, Write};

use crate::database::{connect, get_terms};
use crate::term::Term;

const BASE: &str = "./book/src/";

pub fn run() -> rusqlite::Result<(), Box<dyn std::error::Error>> {
    let conn = connect()?;
    let terms = get_terms(&conn)?;

    setup(&terms)?;
    create_pages(&terms)?;
    Ok(())
}

fn setup(terms: &[Term]) -> Result<(), Box<dyn std::error::Error>> {
    if !std::path::Path::new("./book/src").exists() {
        return Err("Please initialize a mdbook with the folder name: book".into());
    }
    if !std::path::Path::new("./book/terms").exists() {
        std::fs::create_dir("./book/src/terms/")?;
    }
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
        let line = format!("- [{}]({}{})", term.name(), "terms/", term.filename());
        writeln!(writer, "{line}")?;
    }

    writer.flush()?;
    Ok(())
}
fn create_pages(terms: &[Term]) -> Result<(), Box<dyn std::error::Error>> {
    for term in terms {
        create_page(term)?;
    }
    Ok(())
}

fn create_page(term: &Term) -> Result<(), Box<dyn std::error::Error>> {
    let filepath = BASE.to_owned() + "terms/" + &term.filename();
    let file = fs::File::create(&filepath)?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "# {}\n", term.name())?;
    writeln!(writer, "## Book-definition\n")?;
    writeln!(writer, "{}", term.book_definition.trim_end())?;

    writer.flush()?;
    Ok(())
}
