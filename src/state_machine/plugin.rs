use bevy::{
    asset::AssetApp,
    prelude::{App, Assets, Bundle, Component, Handle, Plugin, Query, Res, Update},
    sprite::{SpriteBundle, TextureAtlas},
    time::Time,
};
use serde::{Deserialize, Serialize};

use super::StateInstance;
use crate::{
    state_machine::{AnimationStateMachine, UpdateArgs},
    states::index::{IndexData, IndexState},
};

/// The Dynastes sprite animation plugin for Bevy.
///
/// Updates animation frames for bundles with an `AnimationStateMachine<TexutreAtlasSprite, Handle<TextureAtlas>>`.
#[derive(Default)]
pub struct DynastesAnimationPlugin;

impl Plugin for DynastesAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<AnimationStateMachine<TextureAtlas, IndexState>>();
        // app.init_asset_loader::<AsmLoader>();
        // app.init_asset::<BevyFrameSource>();
        // app.init_asset::<FrameSourceLoader>();
        app.add_systems(Update, animation_system);
    }
}

/// A convenience wrapper for an optional `StateInstance` of a BevyASM
#[derive(Debug, Serialize, Deserialize, Component, Default)]
pub struct MaybeBevyStateInstance(pub Option<StateInstance<IndexState, IndexData>>);

pub type BevyASM = AnimationStateMachine<TextureAtlas, IndexState>;

/// Run the animations across bundles of `BevyASM` and `BevyStateInstance`
pub fn animation_system(
    time: Res<Time>,
    asms: Res<Assets<BevyASM>>,
    mut query: Query<(
        &Handle<BevyASM>,
        &mut TextureAtlas,
        &mut MaybeBevyStateInstance,
    )>,
) {
    for (asm_handle, mut sprite, mut maybe_instance) in query.iter_mut() {
        let asm = asms.get(asm_handle).unwrap();
        let mut instance = maybe_instance.0.get_or_insert(asm.default_instance());
        asm.update(
            &mut instance,
            UpdateArgs {
                delta_ms: time.delta_seconds_f64() * 1000.,
            },
            &mut sprite,
        )
    }
}

#[derive(Bundle)]
/// A Bundle of the components needed to run an animation with Bevy ECS
pub struct DynastesAnimationBundle {
    /// The animation state machine
    pub state_machine: Handle<BevyASM>,
    /// The current state in `state_machine`
    pub animation_state: MaybeBevyStateInstance,
    /// The sprite sheet that the animation is across
    pub sprite_bundle: SpriteBundle,
    /// Texture Atlas
    pub texture_atlas: TextureAtlas,
}
