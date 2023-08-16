use bevy::{
    prelude::Handle,
    sprite::{TextureAtlas, TextureAtlasSprite},
};

use crate::state_machine::{IndexSprite, Sprite};

mod plugin;

pub use plugin::SpriteAnimationPlugin;

impl Sprite for TextureAtlasSprite {
    type FrameSource = Handle<TextureAtlas>;
}

impl IndexSprite for TextureAtlasSprite {
    fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    fn get_index(&self) -> usize {
        self.index
    }
}
