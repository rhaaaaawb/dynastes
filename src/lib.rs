//! A state machine thing for animating sprites with bevy.
// #![deny(warnings)]
#![warn(missing_docs)]
mod state_machine;
mod states;

#[cfg(feature = "bevy")]
mod bevy;
