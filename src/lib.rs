//! A state machine thing for animating sprites with bevy.
// #![deny(warnings)]
#![warn(missing_docs)]
/// The base logic for switching between animation states
pub mod state_machine;
/// The types of states that can be switched between
pub mod states;

pub use state_machine::plugin::*;
