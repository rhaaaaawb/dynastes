use bevy::{
    prelude::{App, Plugin, Query, Res, Update},
    sprite::TextureAtlasSprite,
    time::Time,
};

use crate::{bevy::BevyASM, state_machine::UpdateArgs};

/// The Dynastes sprite animation plugin for Bevy.
///
/// Updates animation frames for bundles with an `AnimationStateMachine<TexutreAtlasSprite, Handle<TextureAtlas>>`.
pub struct SpriteAnimationPlugin;

impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animation_system);
    }
}

/// Run the animations across bundles of `AnimationStateMachine<S>` and `S`
pub fn animation_system(
    time: Res<Time>,
    mut query: Query<(&mut BevyASM, &mut TextureAtlasSprite)>,
) {
    for (mut asm, mut sprite) in query.iter_mut() {
        asm.update(
            UpdateArgs {
                delta_ms: time.delta_seconds_f64() * 1000.,
            },
            &mut sprite,
        )
    }
}
