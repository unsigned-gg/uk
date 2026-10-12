//! Zero-I/O domain models for `uk`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UkState {
    pub version: u32,
    pub id: String,
}

impl Default for UkState {
    fn default() -> Self {
        Self {
            version: 1,
            id: String::from("initial"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_state() {
        let state = UkState::default();
        assert_eq!(state.version, 1);
        assert_eq!(state.id, "initial");
    }
}
