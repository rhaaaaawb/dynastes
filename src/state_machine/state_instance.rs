use core::marker::PhantomData;

use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

use super::{AnimationState, StateID};

#[derive(Debug, Serialize, Deserialize, Component)]
/// The per-instance information necessary for running the ASM
pub struct StateInstance<S, D> {
    /// This instance's ID
    pub current_id: StateID,
    /// The state's data for this instance
    pub data: D,
    phantom: PhantomData<S>,
}

impl<S, D> StateInstance<S, D>
where
    S: AnimationState<Data = D>,
{
    /// Creates a new instance for the state and corresponding data
    pub fn new(state_id: StateID, data: D) -> Self {
        Self {
            current_id: state_id,
            data,
            phantom: PhantomData::default(),
        }
    }
}
