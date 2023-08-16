use std::{collections::HashMap, fmt::Debug};

use bevy::reflect::TypePath;
#[cfg(feature = "bevy")]
use bevy::{
    prelude::{Component, Handle, Query, Reflect, Res},
    reflect::TypeUuid,
    sprite::{TextureAtlas, TextureAtlasSprite},
    time::Time,
    utils::Uuid,
};
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
/// The ID of a state in the state machine
pub struct StateID(String);

impl From<String> for StateID {
    fn from(value: String) -> Self {
        StateID(value)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "bevy", derive(Component))]
/// A finite state machine across animation states
pub struct AnimationStateMachine<S, F> {
    frame_source: F,
    current_id: StateID,
    states: HashMap<StateID, Box<dyn AnimationState<Sprite = S>>>,
}

#[cfg(feature = "bevy")]
pub type BevyASM = AnimationStateMachine<TextureAtlasSprite, Handle<TextureAtlas>>;

impl<S, F> AnimationStateMachine<S, F>
where
    S: Sprite<FrameSource = F>,
{
    /// Creates a new FSM initialized with `default_id` and `default_state`
    pub fn new(
        frame_source: F,
        default_id: StateID,
        default_state: Box<dyn AnimationState<Sprite = S>>,
    ) -> Self {
        let mut states = HashMap::new();
        states.insert(default_id.clone(), default_state);
        Self {
            frame_source,
            current_id: default_id,
            states,
        }
    }

    /// Add all given `StateID` `AnimationState` pairs to the FSM.
    pub fn add_states(&mut self, pairs: Vec<(StateID, Box<dyn AnimationState<Sprite = S>>)>) {
        for (id, state) in pairs {
            self.states.insert(id, state);
        }
    }

    /// Run an update cycle for the FSM, potentially changing the frame or state
    pub fn update(&mut self, args: UpdateArgs, sprite: &mut S) {
        let state = self.states.get_mut(&self.current_id).unwrap();

        state.update(args, sprite);

        if let Some(next_id) = state.next_state() {
            self.set_state(next_id.to_owned());
        }
    }

    /// Set the active state of the FSM
    pub fn set_state(&mut self, new_id: StateID) -> Option<()> {
        if let Some(next) = self.states.get_mut(&new_id) {
            self.current_id = new_id;
            next.start();
            Some(())
        } else {
            error!("Required state '{new_id:?}' does not exist, continuing on current state");
            None
        }
    }
}

#[cfg(feature = "bevy")]
impl<S, F> AnimationStateMachine<S, F>
where
    S: 'static + Component + Sprite<FrameSource = F>,
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

#[cfg(feature = "bevy")]
impl TypePath for BevyASM {
    fn type_path() -> &'static str {
        todo!()
    }

    fn short_type_path() -> &'static str {
        todo!()
    }
}

/// The types of states that can be represented by the AnimationStateMachine
pub trait AnimationState: Debug + Send + Sync {
    /// The "sprite" type that is modified by this state, i.e. `TextureAtlasSprite` for Bevy animation state machines
    type Sprite: Sprite;

    /// Called when the state machine starts processing this state (used for reseting any stateful fields)
    fn start(&mut self);

    /// Update the given sprite according to the behavior of this state.
    fn update(&mut self, args: UpdateArgs, sprite: &mut Self::Sprite);

    /// Queries for the ID of the next state in the state machine.
    /// # Returns
    /// * `None` if the state machine should continue processing this state
    /// * `Some(id)` if the state machine should stop processing this state and move to `id`
    fn next_state(&self) -> Option<StateID>;
}

/// The types that an `AnimationStateMachine` can animate
pub trait Sprite: Debug + IndexSprite {
    type FrameSource: Debug + Send + Sync;
}

/// A sprite whose current frame is modified by setting an index
pub trait IndexSprite: Debug {
    /// Set the frame by changing the index
    fn set_index(&mut self, index: usize);

    /// Get the current frame index for the sprite
    fn get_index(&self) -> usize;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Arguments used when updating the `AnimationStateMachine`
pub struct UpdateArgs {
    /// The number of ms elapsed since the last update was called
    pub delta_ms: f64,
}
