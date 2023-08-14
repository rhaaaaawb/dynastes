use bevy::sprite::TextureAtlasSprite;

use crate::state_machine::{IndexSprite, Sprite};

mod plugin;

pub use plugin::SpriteAnimationPlugin;

impl Sprite for TextureAtlasSprite {}

impl IndexSprite for TextureAtlasSprite {
    fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    fn get_index(&self) -> usize {
        self.index
    }
}
