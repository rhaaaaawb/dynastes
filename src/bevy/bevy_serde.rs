use bevy::sprite::TextureAtlasSprite;
use serde::{Deserialize, Serialize};

use crate::{
    state_machine::{StateContainer, StateID},
    states::index::IndexState,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// A helper struct for serializing and deserializing `BevyASM`
pub struct BevyASMSerde {
    /// The path to the TextureAtlas's `.fs` file
    pub frame_source: String,
    /// The default StateID for new instances
    pub default_id: StateID,
    /// The map of States and StateIDs for the ASM
    pub states: StateContainer<IndexState<TextureAtlasSprite>>,
}
