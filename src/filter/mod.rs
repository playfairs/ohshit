use crate::level::Level;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Filter {
    minimum: Level,
}

impl Filter {
    pub fn new(minimum: Level) -> Self {
        Self { minimum }
    }

    pub fn minimum(&self) -> Level {
        self.minimum
    }

    pub fn allows(&self, level: Level) -> bool {
        level.is_at_least(self.minimum)
    }
}

impl Default for Filter {
    fn default() -> Self {
        Self::new(Level::Info)
    }
}
