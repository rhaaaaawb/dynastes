use core::marker::PhantomData;
use std::{collections::HashMap, fmt::Debug};

use bevy::{
    asset::Asset,
    reflect::{Reflect, TypePath},
};
use serde::{Deserialize, Serialize};

pub mod plugin;
mod state_container;
mod state_id;
mod state_instance;
mod traits;

pub use state_container::StateContainer;
pub use state_id::StateID;
pub use state_instance::StateInstance;
pub use traits::*;

#[derive(Debug, Serialize, Deserialize, Asset, Reflect)]
/// A finite state machine across animation states
pub struct AnimationStateMachine<Sprite: TypePath + Send + Sync, State: TypePath + Send + Sync> {
    default_id: StateID,
    states: StateContainer<State>,
    phantom_sprite: PhantomData<Sprite>,
}

impl<S, T> AnimationStateMachine<S, T>
where
    S: AnimationSprite + TypePath + Send + Sync,
    T: AnimationState<Sprite = S> + TypePath + Send + Sync,
{
    /// Creates a new FSM initialized with `default_id` and `default_state`
    pub fn with_default(default_id: StateID, default_state: T) -> Self {
        let mut states = HashMap::new();
        states.insert(default_id.clone(), default_state);
        Self {
            default_id,
            states: states.into(),
            phantom_sprite: PhantomData::default(),
        }
    }

    /// Creates a new FSM initialized with `default_id` and `default_state`
    pub fn with_states(default_id: StateID, states: StateContainer<T>) -> Self {
        Self {
            default_id,
            states,
            phantom_sprite: PhantomData::default(),
        }
    }

    /// Add all given `StateID` `AnimationState` pairs to the FSM.
    pub fn add_states(&mut self, pairs: Vec<(StateID, T)>) {
        for (id, state) in pairs {
            self.states.0.insert(id, state);
        }
    }

    /// Run an update cycle for the FSM, potentially changing the frame or state
    pub fn update(
        &self,
        instance: &mut StateInstance<T, T::Data>,
        args: UpdateArgs,
        sprite: &mut S,
    ) {
        let state = self.states.0.get(&instance.current_id).unwrap();

        state.update(&mut instance.data, args, sprite);

        if let Some(next_id) = state.next_state(&instance.data, sprite) {
            *instance = self.new_instance(next_id).unwrap();
        }
    }

    /// The ASMs frame source
    // pub fn frame_source(&self) -> &F {
    //     &self.frame_source
    // }

    /// Creates a new instance from the default state
    pub fn default_instance(&self) -> StateInstance<T, T::Data> {
        let state = self.states.0.get(&self.default_id).unwrap();
        StateInstance::new(self.default_id.clone(), state.start())
    }

    /// Creates a new instance from the given state id if it exists
    pub fn new_instance(&self, instance_id: StateID) -> Option<StateInstance<T, T::Data>> {
        self.states
            .0
            .get(&instance_id)
            .map(|state| StateInstance::new(instance_id, state.start()))
    }

    /// The ASMs default state ID
    pub fn default_id(&self) -> &StateID {
        &self.default_id
    }

    /// The ASMs possible states
    pub fn states(&self) -> &StateContainer<T> {
        &self.states
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Arguments used when updating the `AnimationStateMachine`
pub struct UpdateArgs {
    /// The number of ms elapsed since the last update was called
    pub delta_ms: f64,
}
