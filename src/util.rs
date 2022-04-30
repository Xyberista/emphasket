use crate::database::{connect, get_terms};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let conn = connect()?;
    let terms = get_terms(&conn)?;
    let examples_word_count = terms
        .iter()
        .fold(0, |a, b| a + b.example.split_whitespace().count());
    println!("Examples word count: {}", examples_word_count);
    let definititions_word_count = terms
        .iter()
        .fold(0, |a, b| a + b.user_definition.split_whitespace().count());
    println!("User definitions word count: {}", definititions_word_count);
    Ok(())
}
