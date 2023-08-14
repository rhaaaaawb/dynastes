use std::{collections::HashMap, fmt::Debug};

use bevy::{prelude::Res, sprite::TextureAtlasSprite, time::Time};
#[cfg(feature = "bevy")]
use bevy::{
    prelude::{Component, Query, Reflect},
    reflect::{TypePath, TypeUuid},
    utils::Uuid,
};
use log::{error, trace};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
pub struct StateID(String);

impl From<String> for StateID {
    fn from(value: String) -> Self {
        StateID(value)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "bevy", derive(Component))]
pub struct AnimationStateMachine<S> {
    current_id: StateID,
    states: HashMap<StateID, Box<dyn AnimationState<Sprite = S>>>,
}

impl<S> AnimationStateMachine<S>
where
    S: Sprite,
{
    pub fn new(default_id: StateID, default_state: Box<dyn AnimationState<Sprite = S>>) -> Self {
        let mut states = HashMap::new();
        states.insert(default_id.clone(), default_state);
        Self {
            current_id: default_id,
            states,
        }
    }

    pub fn update(&mut self, args: UpdateArgs, sprite: &mut S) {
        let state = self.states.get_mut(&self.current_id).unwrap();

        state.update(args, sprite);

        if let Some(next_id) = state.next_state() {
            self.set_state(next_id.to_owned());
        }
    }

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
impl<S> AnimationStateMachine<S>
where
    S: 'static + Component + Sprite,
{
    pub fn animation_system(time: Res<Time>, mut query: Query<(&mut Self, &mut S)>) {
        for (mut asm, mut sprite) in query.iter_mut() {
            println!("ASM: {:?}, sprite: {:?}", asm, sprite);
            asm.update(
                UpdateArgs {
                    delta_ms: time.delta_seconds_f64() * 1000.,
                },
                &mut sprite,
            )
        }
    }
}

pub trait AnimationState: Debug + Send + Sync {
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

pub trait Sprite: Debug + IndexSprite {}

pub trait IndexSprite: Debug {
    fn set_index(&mut self, index: usize);

    fn get_index(&self) -> usize;
}

// Filler for now
#[derive(Debug, Clone)]
pub struct UpdateArgs {
    /// The number of ms elapsed since the last update was called
    pub delta_ms: f64,
}
