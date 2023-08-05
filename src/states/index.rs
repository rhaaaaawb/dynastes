use core::marker::PhantomData;

use crate::state_machine::{AnimationState, IndexSprite, Sprite, StateID};

#[derive(Debug, Clone)]
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
}

impl<S> IndexState<S> {
    fn increment(&mut self) {
        self.index = if self.index + 1 > self.max_i {
            self.index + 1 - self.max_i + self.min_i
        } else {
            self.index + 1
        }
    }
}

impl<S> AnimationState for IndexState<S>
where
    S: Sprite + IndexSprite,
{
    type Sprite = S;

    fn start(&mut self) {
        if !self.maintain_index {
            self.index = self.min_i;
        }
    }

    fn update(&mut self, args: crate::state_machine::UpdateArgs, sprite: &mut Self::Sprite) {
        // TODO! some sort of logic for frame time
        let next_frame = true;
        if (next_frame) {
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
