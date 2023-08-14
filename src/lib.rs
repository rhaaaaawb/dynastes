//! A state machine thing for animating sprites with bevy.
// #![deny(warnings)]
#![warn(missing_docs)]
pub mod state_machine;
pub mod states;

#[cfg(feature = "bevy")]
pub mod bevy;
