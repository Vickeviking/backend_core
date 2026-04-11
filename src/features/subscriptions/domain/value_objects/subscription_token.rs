#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscriptionToken(String);

impl SubscriptionToken {
    pub fn new(value: String) -> Self {
        Self(value)
    }
}

impl AsRef<str> for SubscriptionToken {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for SubscriptionToken {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl std::fmt::Display for SubscriptionToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
