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
    /// The average framerate of the animation (ignoring fluidity)
    nominal_mspf: f64,
    /// The number of milliseconds each frame should stay on screen for
    actual_mspf: f64,
    /// The state to switch to after reading the max sprite index
    /// If `None` loop on this state indefinitely.
    next_state: Option<StateID>,
    /// The "phase shift" of the animation in ms
    phase: f64,
    frames_per_increment: f64,
    #[serde(skip)]
    phantom: PhantomData<Sprite>,
}

impl<S> IndexState<S> {
    /// Make a new index state
    /// * `min_i` The minimum index in the sprite sheet that this state should use (inclusive).
    /// * `max_i` The maximum index in the sprite sheet that this state should use (inclusive).
    /// * `mspf` The "average" frame rate of the animation.
    ///         When `fluidity_factor` is 1, this is the number of milliseconds that a single frame is rendered
    /// * `next_state` If `Some` the state to switch to after reaching `max_i`, otherwise loop on this state.
    /// * `phase` If `Some` the phase shift of the animation in frames (default 0).
    /// * `fluidity_factor` From (0, 1] the fluidity of the animation as a whole (default 1).
    pub fn new(
        min_i: usize,
        max_i: usize,
        nominal_mspf: f64,
        next_state: Option<StateID>,
        phase: Option<f64>,
        fluidity_factor: Option<f64>,
    ) -> Self {
        let fluidity_factor = fluidity_factor.unwrap_or_else(|| 1.);
        if fluidity_factor > 1. || fluidity_factor <= 0. {
            panic!("Animation fluidity factor must be in the range (0, 1]");
        }

        let actual_mspf = nominal_mspf / fluidity_factor;
        println!("{actual_mspf}");
        let frames_per_increment = 1. / fluidity_factor;
        println!("{frames_per_increment}");

        Self {
            min_i,
            max_i,
            nominal_mspf,
            actual_mspf,
            next_state,
            phase: phase.unwrap_or_default(),
            frames_per_increment,
            phantom: PhantomData::default(),
        }
    }

    fn maybe_increment(&self, data: &mut IndexData<S>) {
        let mut effective_time_elapsed = data.ms_elapsed;

        if data.phase_delay > 0. {
            if !(data.ms_elapsed > data.phase_delay) {
                return;
            }
            effective_time_elapsed -= data.phase_delay;
            // Don't delay future frames in this state
            data.phase_delay = 0.;
        }

        let nominal_num_frames = f64::floor(effective_time_elapsed / self.nominal_mspf);
        // If we checked for reaching the end based on the actual frame it would lead to completly fluid
        // animations reaching the end some number of frames early.
        // Instead we check if the animation is at the end based on if it were running completely fluidly
        let nominal_next_index = data.index + nominal_num_frames as usize;
        if nominal_next_index >= self.max_i {
            data.reached_end = true;
        }

        let num_frames = ((nominal_num_frames / self.frames_per_increment).floor()
            * self.frames_per_increment) as usize;
        data.ms_elapsed %= self.actual_mspf;

        let next_index = data.index + num_frames;
        data.index = if next_index > self.max_i {
            next_index - self.max_i + self.min_i - 1
        } else {
            next_index
        };
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
        self.maybe_increment(data);
        sprite.set_index(data.index);
    }

    fn next_state(&self, data: &Self::Data) -> Option<StateID> {
        self.next_state.as_ref().and_then(|next| {
            if data.index >= self.max_i || data.reached_end {
                Some(next.clone())
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
    /// The number of ms to "wait" before updating the state for the first time
    pub phase_delay: f64,
    reached_end: bool,
    #[serde(skip)]
    phantom: PhantomData<Sprite>,
}

impl<S> IndexData<S> {
    /// Creates the data from a given state
    pub fn new(state: &IndexState<S>) -> Self {
        Self {
            index: state.min_i,
            ms_elapsed: 0.,
            phase_delay: state.phase,
            reached_end: false,
            phantom: PhantomData::default(),
        }
    }
}
