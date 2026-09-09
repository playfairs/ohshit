use crate::diagnostic::action::Action;
use crate::diagnostic::cause::Cause;
use crate::diagnostic::location::Location;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostic {
    headline: String,
    cause: Cause,
    reason: Option<String>,
    action: Action,
    location: Option<Location>,
    context: Vec<(String, String)>,
}

impl Diagnostic {
    pub fn new(headline: impl Into<String>) -> Self {
        Self {
            headline: headline.into(),
            cause: Cause::none(),
            reason: None,
            action: Action::none(),
            location: None,
            context: Vec::new(),
        }
    }

    pub fn headline(&self) -> &str {
        &self.headline
    }

    pub fn with_cause(mut self, cause: impl Into<String>) -> Self {
        self.cause = Cause::new(cause);
        self
    }

    pub fn cause(&self) -> Option<&str> {
        self.cause.message()
    }

    pub fn with_reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    pub fn with_action(mut self, action: impl Into<String>) -> Self {
        self.action = Action::new(action);
        self
    }

    pub fn action(&self) -> Option<&str> {
        self.action.message()
    }

    pub fn with_location(mut self, location: Location) -> Self {
        self.location = Some(location);
        self
    }

    pub fn location(&self) -> Option<&Location> {
        self.location.as_ref()
    }

    pub fn with_context(mut self, key: impl Into<String>, value: impl std::fmt::Display) -> Self {
        self.context.push((key.into(), value.to_string()));
        self
    }

    pub fn context(&self) -> &[(String, String)] {
        &self.context
    }

    pub fn render(&self) -> String {
        let mut lines = vec!["OH SHIT".to_string(), String::new(), self.headline.clone()];

        if let Some(location) = &self.location {
            lines.push(String::new());
            lines.push("Location:".to_string());
            lines.push(format!("  {}:{}", location.file(), location.line()));
        }

        if let Some(cause) = self.cause() {
            lines.push(String::new());
            lines.push("Cause:".to_string());
            lines.push(format!("  {cause}"));
        }

        if let Some(reason) = self.reason() {
            lines.push(String::new());
            lines.push("Reason:".to_string());
            lines.push(format!("  {reason}"));
        }

        if !self.context.is_empty() {
            lines.push(String::new());
            lines.push("Additional information:".to_string());
            for (key, value) in &self.context {
                lines.push(format!("  {key}: {value}"));
            }
        }

        if let Some(action) = self.action() {
            lines.push(String::new());
            lines.push("Try:".to_string());
            lines.push(format!("  {action}"));
        }

        lines.join("\n")
    }
}
