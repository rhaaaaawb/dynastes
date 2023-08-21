use std::path::{Path, PathBuf};

use bevy::{
    prelude::{AssetServer, Component, Res, Vec2},
    reflect::{TypePath, TypeUuid},
    sprite::{TextureAtlas, TextureAtlasSprite},
};
use serde::{Deserialize, Serialize};

use crate::{
    state_machine::{AnimationStateMachine, IndexSprite, Sprite, StateID},
    states::IndexState,
};

mod plugin;

pub use plugin::SpriteAnimationPlugin;

/// A convenience wrapper for the bevy monomorphization of the ASM
#[derive(Debug, Serialize, Deserialize, Component, TypeUuid, TypePath)]
#[uuid = "74377e21-153d-4e30-9b5e-1b857a9ab807"]
pub struct BevyASM(
    pub AnimationStateMachine<TextureAtlasSprite, IndexState<TextureAtlasSprite>, BevyFrameSource>,
);

impl BevyASM {
    /// Creates a new Bevy ASM initialized with `default_id` and `default_state`
    pub fn new(
        frame_source: BevyFrameSource,
        default_id: StateID,
        default_state: IndexState<TextureAtlasSprite>,
    ) -> Self {
        BevyASM(AnimationStateMachine::new(
            frame_source,
            default_id,
            default_state,
        ))
    }
}

impl Sprite for TextureAtlasSprite {}

impl IndexSprite for TextureAtlasSprite {
    fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    fn get_index(&self) -> usize {
        self.index
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// The source of the `BevyASM`'s frame
pub struct BevyFrameSource {
    /// The asset path to the ASM's sprite sheet
    pub path: PathBuf,
    /// Metadata for constructing the texture atlas from the sprite sheet
    pub metadata: TextureAtlasGridMetadata,
}

impl BevyFrameSource {
    /// Loads the referenced sprite sheet and converts it to a texture atlas
    pub fn make_texture_atlas(&self, server: Res<AssetServer>) -> TextureAtlas {
        let handle = server.load::<_, &Path>(self.path.as_ref());
        TextureAtlas::from_grid(
            handle,
            self.metadata.tile_size,
            self.metadata.columns,
            self.metadata.rows,
            self.metadata.padding,
            self.metadata.offset,
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Metadata needed for constructing a grided texture atlas from a sprite sheet
pub struct TextureAtlasGridMetadata {
    /// The size of each sprite in pixels
    pub tile_size: Vec2,
    /// The number of columns in the sprite sheet
    pub columns: usize,
    /// The number of rows in the sprite sheet
    pub rows: usize,
    /// Separation between each sprite
    pub padding: Option<Vec2>,
    /// Where the grid starts relative to the top left corner
    pub offset: Option<Vec2>,
}
