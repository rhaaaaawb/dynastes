use core::marker::PhantomData;
use std::{collections::HashMap, fmt::Debug};

use log::error;
use serde::{Deserialize, Serialize};

mod state_container;
mod state_id;
mod traits;

pub use state_container::StateContainer;
pub use state_id::StateID;
pub use traits::*;

#[derive(Debug, Serialize, Deserialize)]
/// A finite state machine across animation states
pub struct AnimationStateMachine<Sprite, State, FrameSource> {
    frame_source: FrameSource,
    current_id: StateID,
    states: StateContainer<State>,
    #[serde(skip)]
    phantom: PhantomData<Sprite>,
}

impl<S, T, F> AnimationStateMachine<S, T, F>
where
    S: Sprite,
    T: AnimationState<Sprite = S>,
{
    /// Creates a new FSM initialized with `default_id` and `default_state`
    pub fn new(frame_source: F, default_id: StateID, default_state: T) -> Self {
        let mut states = HashMap::new();
        states.insert(default_id.clone(), default_state);
        Self {
            frame_source,
            current_id: default_id,
            states: states.into(),
            phantom: PhantomData::default(),
        }
    }

    /// Add all given `StateID` `AnimationState` pairs to the FSM.
    pub fn add_states(&mut self, pairs: Vec<(StateID, T)>) {
        for (id, state) in pairs {
            self.states.0.insert(id, state);
        }
    }

    /// Run an update cycle for the FSM, potentially changing the frame or state
    pub fn update(&mut self, args: UpdateArgs, sprite: &mut S) {
        let state = self.states.0.get_mut(&self.current_id).unwrap();

        state.update(args, sprite);

        if let Some(next_id) = state.next_state() {
            self.set_state(next_id.to_owned());
        }
    }

    /// Set the active state of the FSM
    pub fn set_state(&mut self, new_id: StateID) -> Option<()> {
        if let Some(next) = self.states.0.get_mut(&new_id) {
            self.current_id = new_id;
            next.start();
            Some(())
        } else {
            error!("Required state '{new_id:?}' does not exist, continuing on current state");
            None
        }
    }

    /// The ASMs frame source
    pub fn frame_source(&self) -> &F {
        &self.frame_source
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Arguments used when updating the `AnimationStateMachine`
pub struct UpdateArgs {
    /// The number of ms elapsed since the last update was called
    pub delta_ms: f64,
}
