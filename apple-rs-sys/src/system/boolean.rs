use std::{fmt::Display, ops::Not};

use crate::utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Boolean(pub bool);

impl TryFrom<UnsafeBool> for Boolean {
    type Error = ();

    fn try_from(obj: UnsafeBool) -> Result<Self, Self::Error> {
        let data = match obj {
            runtime::YES => true,
            runtime::NO => false,
        };

        Ok(Boolean(data))
    }
}

impl Into<UnsafeBool> for Boolean {
    fn into(self) -> UnsafeBool {
        if self.0 {
            runtime::YES
        } else {
            runtime::NO
        }
    }
}

impl Not for Boolean {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Boolean {
    pub fn id(self) -> UnsafeBool {
        self.into()
    }

    pub fn new(value: bool) -> Self {
        Self(value)
    }

    pub fn value(&self) -> bool {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use objc::runtime;

    use super::Boolean;

    #[test]
    fn test_boolean() {
        let boolean = Boolean::new(true);
        assert_eq!(boolean.value(), true);
        assert_eq!(boolean.id(), runtime::YES);

        let boolean = Boolean::new(false);
        assert_eq!(boolean.value(), false);
        assert_eq!(boolean.id(), runtime::NO);
    }
}