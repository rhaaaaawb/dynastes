use core::marker::PhantomData;

use crate::state_machine::{AnimationState, IndexSprite, Sprite, StateID};

#[derive(Debug, Clone)]
/// A state that determines the frame based on an incrementing index
pub struct IndexState<Sprite> {
    min_i: usize,
    max_i: usize,
    /// The number of milliseconds each frame should stay on screen for
    mspf: f64,
    /// The state to switch to after reading the max sprite index
    /// If `None` loop on this state indefinitely.
    next_state: Option<StateID>,
    /// True when the current index should be maintained after restarting the state
    maintain_index: bool,
    phantom: PhantomData<Sprite>,
    index: usize,
    /// The total number of milliseconds that have passed since the last frame update
    ms_elapsed: f64,
}

impl<S> IndexState<S> {
    /// Make a new index state
    /// * `min_i` The minimum index in the sprite sheet that this state should use
    /// * `max_i` The maximum index in the sprite sheet that this state should use
    /// * `mspf` The number of milliseconds that each frame should be on screen
    /// * `next_state` If `Some` the state to switch to after reaching `max_i`, otherwise loop on this state
    /// * `maintain_index` If true the current index will persist after switching off and back to this state 
    pub fn new(
        min_i: usize,
        max_i: usize,
        mspf: f64,
        next_state: Option<StateID>,
        maintain_index: bool,
    ) -> Self {
        Self {
            min_i,
            max_i,
            mspf,
            next_state,
            maintain_index,
            phantom: PhantomData::default(),
            index: min_i,
            ms_elapsed: 0.,
        }
    }

    fn increment(&mut self) {
        let num_frames = f64::floor(self.ms_elapsed / self.mspf) as usize;
        self.ms_elapsed %= self.mspf;
        self.index = if self.index + num_frames > self.max_i {
            self.index + num_frames - self.max_i + self.min_i
        } else {
            self.index + num_frames
        }
    }
}

impl<S> AnimationState for IndexState<S>
where
    S: Send + Sync + Sprite + IndexSprite,
{
    type Sprite = S;

    fn start(&mut self) {
        if !self.maintain_index {
            self.index = self.min_i;
        }
    }

    fn update(&mut self, args: crate::state_machine::UpdateArgs, sprite: &mut Self::Sprite) {
        self.ms_elapsed += args.delta_ms;
        if self.ms_elapsed >= self.mspf {
            self.increment();
            sprite.set_index(self.index);
        }
    }

    fn next_state(&self) -> Option<StateID> {
        self.next_state.as_ref().and_then(|n| {
            if self.index >= self.max_i {
                Some(n.clone())
            } else {
                None
            }
        })
    }
}
