#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Context {
    entries: Vec<(String, String)>,
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, key: impl Into<String>, value: impl std::fmt::Display) {
        self.entries.push((key.into(), value.to_string()));
    }

    pub fn with(mut self, key: impl Into<String>, value: impl std::fmt::Display) -> Self {
        self.insert(key, value);
        self
    }

    pub fn entries(&self) -> &[(String, String)] {
        &self.entries
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl From<Vec<(String, String)>> for Context {
    fn from(entries: Vec<(String, String)>) -> Self {
        Self { entries }
    }
}
