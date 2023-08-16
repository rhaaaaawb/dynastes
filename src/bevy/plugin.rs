use bevy::prelude::{App, Plugin, Update};

use crate::state_machine::BevyASM;

/// The Dynastes sprite animation plugin for Bevy.
///
/// Updates animation frames for bundles with an `AnimationStateMachine<TexutreAtlasSprite, Handle<TextureAtlas>>`.
pub struct SpriteAnimationPlugin;

impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, BevyASM::animation_system);
    }
}
