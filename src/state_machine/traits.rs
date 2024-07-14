use std::fmt::Debug;

use bevy::sprite::TextureAtlas;

use super::{StateID, UpdateArgs};

/// The types of states that can be represented by the AnimationStateMachine
pub trait AnimationState: Debug + Send + Sync {
    /// The "sprite" type that is modified by this state, i.e. `TextureAtlasSprite` for Bevy animation state machines
    type Sprite: AnimationSprite;
    /// The data specific to each instance of this state
    type Data;

    /// Called when the state machine starts processing this state (used for reseting any stateful fields)
    fn start(&self) -> Self::Data;

    /// Update the given sprite according to the behavior of this state.
    fn update(&self, data: &mut Self::Data, args: UpdateArgs, sprite: &mut Self::Sprite);

    /// Queries for the ID of the next state in the state machine.
    /// # Returns
    /// * `None` if the state machine should continue processing this state
    /// * `Some(id)` if the state machine should stop processing this state and move to `id`
    fn next_state(&self, data: &Self::Data, sprite: &Self::Sprite) -> Option<StateID>;
}

/// The types that an `AnimationStateMachine` can animate.
pub trait AnimationSprite: Debug {
    /// Set the frame by changing the index
    fn set_index(&mut self, index: usize);

    /// Get the current frame index for the sprite
    fn get_index(&self) -> usize;
}

impl AnimationSprite for TextureAtlas {
    /// Set the frame by changing the index
    fn set_index(&mut self, index: usize) {
        self.index = index
    }

    /// Get the current frame index for the sprite
    fn get_index(&self) -> usize {
        self.index
    }
}
