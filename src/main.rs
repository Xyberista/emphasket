use rusqlite::Result;

mod user_interface;
pub mod database;

#[derive(Debug)]
pub struct Term {
    pub term: String,
    pub book_definition: String,
    pub user_definition: String,
}

fn main() -> Result<()> {
    let conn= database::connect()?;
    user_interface::run(conn)?;
    Ok(())
}
