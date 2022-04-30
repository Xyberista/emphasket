#[derive(Debug)]
pub struct Term {
    pub id: usize,
    pub term: String,
    pub book_definition: String,
    pub user_definition: String,
    pub example: String,
    pub picture_link: String,
}

impl Term {
    pub fn name(&self) -> &str {
        &self.term
    }

    pub fn filename(&self) -> String {
        format!("{}_{}{}", self.id, self.term.replace(' ', "_"), ".md")
    }

    pub fn picture_link(&self) -> String {
        self.picture_link.trim().to_string()
    }

    pub fn picture_name(&self, extension: &str) -> String {
        format!("{}_{}.{}", self.id, self.term.replace(' ', "_"), extension)
    }

    pub fn picture_extension(&self) -> String {
        self.picture_link.rsplit('.').next().unwrap().to_string()
    }
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Term:\n{}", self.term)?;
        writeln!(f, "Book Definition:\n{:?}", self.book_definition)?;
        writeln!(f, "User Definition:\n{}", self.user_definition)?;
        writeln!(f, "Example:\n{}", self.example)?;
        Ok(())
    }
}
