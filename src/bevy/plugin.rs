use bevy::{
    prelude::{AddAsset, App, Assets, Handle, Plugin, Query, Res, Update},
    sprite::TextureAtlasSprite,
    time::Time,
};

use crate::{bevy::BevyASM, state_machine::UpdateArgs};

use super::BevyStateInstance;

/// The Dynastes sprite animation plugin for Bevy.
///
/// Updates animation frames for bundles with an `AnimationStateMachine<TexutreAtlasSprite, Handle<TextureAtlas>>`.
pub struct SpriteAnimationPlugin;

impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<BevyASM>();
        app.add_systems(Update, animation_system);
    }
}

/// Run the animations across bundles of `AnimationStateMachine<S>` and `S`
pub fn animation_system(
    time: Res<Time>,
    asms: Res<Assets<BevyASM>>,
    mut query: Query<(
        &Handle<BevyASM>,
        &mut TextureAtlasSprite,
        &mut BevyStateInstance,
    )>,
) {
    for (asm_handle, mut sprite, mut instance) in query.iter_mut() {
        asms.get(&asm_handle).unwrap().0.update(
            &mut instance,
            UpdateArgs {
                delta_ms: time.delta_seconds_f64() * 1000.,
            },
            &mut sprite,
        )
    }
}
