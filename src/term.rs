#[derive(Debug)]
pub struct Term {
    pub id: usize,
    pub term: String,
    pub book_definition: String,
    pub user_definition: String,
}

impl Term {
    pub fn name(&self) -> &str {
        &self.term
    }

    pub fn filename(&self) -> String {
        self.term.replace(' ', "_")
    }
}
