#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Action {
    message: Option<String>,
}

impl Action {
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
