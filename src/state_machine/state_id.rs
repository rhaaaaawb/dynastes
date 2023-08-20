#[cfg(feature = "bevy")]
use bevy::{
    prelude::{Component, Handle, Query, Reflect, Res},
    reflect::TypeUuid,
    sprite::{TextureAtlas, TextureAtlasSprite},
    time::Time,
    utils::Uuid,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
/// The ID of a state in the state machine
pub struct StateID(pub String);

impl From<String> for StateID {
    fn from(value: String) -> Self {
        StateID(value)
    }
}
