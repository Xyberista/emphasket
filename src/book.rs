use std::fs;
use std::io::{BufWriter, Write};

use crate::database::{connect, get_terms};
use crate::term::Term;

const BASE: &str = "./book/src/";

pub fn run() -> rusqlite::Result<(), Box<dyn std::error::Error>> {
    let conn = connect()?;
    let terms = get_terms(&conn)?;

    copy_figures()?;
    setup(&terms)?;
    create_pages(&terms)?;
    Ok(())
}

fn copy_figures() -> Result<(), Box<dyn std::error::Error>> {
    for file in std::fs::read_dir("./figures/")? {
        let file = file?;
        let destination = format!(
            "{}{}",
            "./book/src/figures/",
            file.file_name().into_string().unwrap()
        );
        std::fs::copy(file.path(), destination)?;
    }
    Ok(())
}

fn setup(terms: &[Term]) -> Result<(), Box<dyn std::error::Error>> {
    if !std::path::Path::new("./book/src").exists() {
        return Err("Please initialize a mdbook with the folder name: book".into());
    }
    if !std::path::Path::new("./book/src/terms").exists() {
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
        let line = format!("- [{}](terms/{})", term.name(), term.filename());
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
    writeln!(writer, "## Book definition\n")?;
    writeln!(writer, "{}\n", term.book_definition.trim_end())?;
    writeln!(writer, "## User definition\n")?;
    writeln!(writer, "{}\n", term.user_definition.trim_end())?;
    writeln!(writer, "## Picture\n")?;
    if !term.picture_link().is_empty() {
        let picture_name = term.picture_name(&term.picture_extension());
        writeln!(writer, "![{}](./../figures/{})", term.name(), picture_name)?;
    }
    writeln!(writer, "## Example\n")?;
    writeln!(writer, "{}", term.example.trim_end())?;

    writer.flush()?;
    Ok(())
}
