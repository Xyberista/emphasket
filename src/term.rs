use std::fmt;

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

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}