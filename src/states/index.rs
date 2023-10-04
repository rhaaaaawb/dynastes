use core::marker::PhantomData;

#[cfg(feature = "bevy")]
use bevy::{prelude::Component, reflect::TypePath};
use serde::{Deserialize, Serialize};

use crate::state_machine::{AnimationState, IndexSprite, Sprite, StateID};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "bevy", derive(TypePath))]

/// A state that determines the frame based on an incrementing index
pub struct IndexState<Sprite> {
    min_i: usize,
    max_i: usize,
    /// The number of milliseconds each frame should stay on screen for
    mspf: f64,
    /// The state to switch to after reading the max sprite index
    /// If `None` loop on this state indefinitely.
    next_state: Option<StateID>,
    #[serde(skip)]
    phantom: PhantomData<Sprite>,
}

impl<S> IndexState<S> {
    /// Make a new index state
    /// * `min_i` The minimum index in the sprite sheet that this state should use
    /// * `max_i` The maximum index in the sprite sheet that this state should use
    /// * `mspf` The number of milliseconds that each frame should be on screen
    /// * `next_state` If `Some` the state to switch to after reaching `max_i`, otherwise loop on this state
    pub fn new(min_i: usize, max_i: usize, mspf: f64, next_state: Option<StateID>) -> Self {
        Self {
            min_i,
            max_i,
            mspf,
            next_state,
            phantom: PhantomData::default(),
        }
    }

    fn increment(&self, data: &mut IndexData<S>) {
        let num_frames = f64::floor(data.ms_elapsed / self.mspf) as usize;
        data.ms_elapsed %= self.mspf;
        data.index = if data.index + num_frames > self.max_i {
            data.index + num_frames - self.max_i + self.min_i
        } else {
            data.index + num_frames
        }
    }
}

impl<S> AnimationState for IndexState<S>
where
    S: Send + Sync + Sprite + IndexSprite,
{
    type Sprite = S;
    type Data = IndexData<S>;

    fn start(&self) -> Self::Data {
        IndexData::new(&self)
    }

    fn update(
        &self,
        data: &mut Self::Data,
        args: crate::state_machine::UpdateArgs,
        sprite: &mut Self::Sprite,
    ) {
        data.ms_elapsed += args.delta_ms;
        if data.ms_elapsed >= self.mspf {
            self.increment(data);
            sprite.set_index(data.index);
        }
    }

    fn next_state(&self, data: &Self::Data) -> Option<StateID> {
        self.next_state.as_ref().and_then(|n| {
            if data.index >= self.max_i {
                Some(n.clone())
            } else {
                None
            }
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "bevy", derive(Component))]
/// The per-instance data of an `IndexState`
pub struct IndexData<Sprite> {
    /// The current index of the state
    pub index: usize,
    /// The total number of milliseconds that have passed since the last frame update
    pub ms_elapsed: f64,
    #[serde(skip)]
    phantom: PhantomData<Sprite>,
}

impl<S> IndexData<S> {
    /// Creates the data from a given state
    pub fn new(state: &IndexState<S>) -> Self {
        Self {
            index: state.min_i,
            ms_elapsed: 0.,
            phantom: PhantomData::default(),
        }
    }
}
