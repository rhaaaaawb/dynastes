use std::collections::HashMap;

pub struct StateID(String);

pub struct AnimationStateMachine<S> {
    current_state: StateID,
    states: HashMap<StateID, Box<dyn AnimationState<Sprite = S>>>,
}

impl<S> AnimationStateMachine<S> {}

pub trait AnimationState {
    type Sprite;

    /// Called when the state machine starts processing this state
    fn start(&mut self);

    /// Update the given sprite according to the behavior of this state.
    fn update(&mut self, update_args: UpdateArgs, sprite: &mut Self::Sprite);

    /// Queries for the ID of the next state in the state machine.
    /// # Returns
    /// * `None` if the state machine should continue processing this state
    /// * `Some(id)` if the state machine should stop processing this state and move to `id`
    fn next_state(&self) -> Option<StateID>;
}

// Filler for now
pub struct UpdateArgs {}
