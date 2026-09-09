#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Location {
    file: String,
    line: u32,
    column: Option<u32>,
}

impl Location {
    pub fn new(file: impl Into<String>, line: u32) -> Self {
        Self {
            file: file.into(),
            line,
            column: None,
        }
    }

    pub fn with_column(mut self, column: u32) -> Self {
        self.column = Some(column);
        self
    }

    pub fn file(&self) -> &str {
        &self.file
    }

    pub fn line(&self) -> u32 {
        self.line
    }

    pub fn column(&self) -> Option<u32> {
        self.column
    }
}
