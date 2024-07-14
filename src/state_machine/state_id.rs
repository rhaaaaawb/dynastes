use bevy::prelude::{Component, Reflect};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize, Component, Reflect)]
/// The ID of a state in the state machine
pub struct StateID(pub String);

impl From<String> for StateID {
    fn from(value: String) -> Self {
        StateID(value)
    }
}
