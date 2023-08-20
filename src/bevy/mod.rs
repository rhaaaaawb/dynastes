use std::path::{Path, PathBuf};

use bevy::{
    prelude::{AssetServer, Res, Vec2},
    sprite::{TextureAtlas, TextureAtlasSprite},
};
use serde::{Deserialize, Serialize};

use crate::state_machine::{IndexSprite, Sprite};

mod plugin;

pub use plugin::SpriteAnimationPlugin;

impl Sprite for TextureAtlasSprite {
    // It must be a pathbuf to a sprite sheet
    type FrameSource = BevyFrameSource;
}

impl IndexSprite for TextureAtlasSprite {
    fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    fn get_index(&self) -> usize {
        self.index
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BevyFrameSource {
    pub path: PathBuf,
    pub metadata: TextureAtlasGridMetadata,
}

impl BevyFrameSource {
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
pub struct TextureAtlasGridMetadata {
    pub tile_size: Vec2,
    pub columns: usize,
    pub rows: usize,
    pub padding: Option<Vec2>,
    pub offset: Option<Vec2>,
}
