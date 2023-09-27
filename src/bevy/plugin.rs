use bevy::{
    prelude::{AddAsset, App, Assets, Handle, Plugin, Query, Res, Update},
    sprite::TextureAtlasSprite,
    time::Time,
};

use crate::{
    bevy::{
        loader::{AsmLoader, FrameSourceLoader},
        BevyASM, BevyFrameSource, MaybeBevyStateInstance,
    },
    state_machine::UpdateArgs,
};

/// The Dynastes sprite animation plugin for Bevy.
///
/// Updates animation frames for bundles with an `AnimationStateMachine<TexutreAtlasSprite, Handle<TextureAtlas>>`.
pub struct SpriteAnimationPlugin;

impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<BevyASM>();
        app.init_asset_loader::<AsmLoader>();
        app.add_asset::<BevyFrameSource>();
        app.init_asset_loader::<FrameSourceLoader>();
        app.add_systems(Update, animation_system);
    }
}

/// Run the animations across bundles of `BevyASM` and `BevyStateInstance`
pub fn animation_system(
    time: Res<Time>,
    asms: Res<Assets<BevyASM>>,
    mut query: Query<(
        &Handle<BevyASM>,
        &mut TextureAtlasSprite,
        &mut MaybeBevyStateInstance,
    )>,
) {
    for (asm_handle, mut sprite, mut maybe_instance) in query.iter_mut() {
        let asm = asms.get(&asm_handle).unwrap();
        let instance = maybe_instance.0.get_or_insert(asm.default_instance());
        asm.0.update(
            &mut instance.0,
            UpdateArgs {
                delta_ms: time.delta_seconds_f64() * 1000.,
            },
            &mut sprite,
        )
    }
}
