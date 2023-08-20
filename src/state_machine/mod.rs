use core::marker::PhantomData;
use std::{collections::HashMap, fmt::Debug, path::PathBuf};

#[cfg(feature = "bevy")]
use bevy::{
    asset::AssetPath,
    prelude::{Component, Handle, Query, Reflect, Res},
    reflect::{TypePath, TypeUuid},
    sprite::{TextureAtlas, TextureAtlasSprite},
    time::Time,
    utils::Uuid,
};
use log::error;
use serde::{Deserialize, Serialize};

use crate::states::IndexState;

mod state_container;
mod state_id;
mod traits;

pub use state_container::StateContainer;
pub use state_id::StateID;
pub use traits::*;

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "bevy", derive(Component, TypePath))]
// #[uuid = "74377e21-153d-4e30-9b5e-1b857a9ab807"]
/// A finite state machine across animation states
pub struct AnimationStateMachine<Sprite, State, FrameSource> {
    frame_source: FrameSource,
    current_id: StateID,
    states: StateContainer<State>,
    #[serde(skip)]
    phantom: PhantomData<Sprite>,
}

#[cfg(feature = "bevy")]
pub type BevyASM = AnimationStateMachine<
    TextureAtlasSprite,
    IndexState<TextureAtlasSprite>,
    crate::bevy::BevyFrameSource,
>;

impl<S, T, F> AnimationStateMachine<S, T, F>
where
    S: Sprite<FrameSource = F>,
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

    pub fn frame_source(&self) -> &F {
        &self.frame_source
    }
}

#[cfg(feature = "bevy")]
impl<S, T, F> AnimationStateMachine<S, T, F>
where
    S: 'static + Component + Sprite<FrameSource = F>,
    T: 'static + Send + Sync + AnimationState<Sprite = S>,
    F: 'static + Send + Sync,
{
    /// Run the animations across bundles of `AnimationStateMachine<S>` and `S`
    pub fn animation_system(time: Res<Time>, mut query: Query<(&mut Self, &mut S)>) {
        for (mut asm, mut sprite) in query.iter_mut() {
            asm.update(
                UpdateArgs {
                    delta_ms: time.delta_seconds_f64() * 1000.,
                },
                &mut sprite,
            )
        }
    }
}

#[cfg(feature = "bevy")]
impl TypeUuid for BevyASM {
    const TYPE_UUID: Uuid = Uuid::from_bytes([
        0x74, 0x37, 0x7e, 0x21, 0x15, 0x3d, 0x4e, 0x30, 0x9b, 0x5e, 0x1b, 0x85, 0x7a, 0x9a, 0xb8,
        0x07,
    ]);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Arguments used when updating the `AnimationStateMachine`
pub struct UpdateArgs {
    /// The number of ms elapsed since the last update was called
    pub delta_ms: f64,
}
