use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::state_id::StateID;

#[derive(Debug, Clone)]
/// A convenience wrapper to hold an ASM's states with their IDs
pub struct StateContainer<S>(pub HashMap<StateID, S>);

impl<S> From<HashMap<StateID, S>> for StateContainer<S> {
    fn from(value: HashMap<StateID, S>) -> Self {
        StateContainer(value)
    }
}

impl<State> Serialize for StateContainer<State>
where
    State: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.iter().collect::<Vec<_>>().serialize(serializer)
    }
}

impl<'de, State> Deserialize<'de> for StateContainer<State>
where
    State: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let as_vec = Vec::deserialize(deserializer)?;
        let mut map = HashMap::with_capacity(as_vec.len());
        for (key, value) in as_vec {
            map.insert(key, value);
        }
        Ok(StateContainer(map))
    }
}
