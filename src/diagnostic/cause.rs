#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Cause {
    message: Option<String>,
}

impl Cause {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: Some(message.into()),
        }
    }

    pub fn none() -> Self {
        Self { message: None }
    }

    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
