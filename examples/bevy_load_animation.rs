use std::{fs, path::Path};

use bevy::prelude::*;
use dynastes::{
    bevy::{BevyFrameSource, SpriteAnimationPlugin, TextureAtlasGridMetadata},
    state_machine::{AnimationStateMachine, BevyASM, StateID},
    states::IndexState,
};

fn main() {
    env_logger::init();

    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin {
            default_sampler: bevy::render::texture::ImageSampler::nearest_descriptor(),
        }))
        .add_plugins(SpriteAnimationPlugin)
        .add_systems(Startup, setup_animations)
        .run()
}

fn setup_animations(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut sprites: ResMut<Assets<TextureAtlas>>,
    // mut state_machines: ResMut<Assets<BevyASM>>,
) {
    commands.spawn(Camera2dBundle::default());

    let asm_str = fs::read_to_string("assets/state-machine.ron").unwrap();
    let asm: BevyASM = ron::from_str(&asm_str).unwrap();

    let texture_atlas_handle = sprites.add(asm.frame_source().make_texture_atlas(asset_server));

    // state_machines.add(asm);

    commands.spawn((
        SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: texture_atlas_handle,
            ..Default::default()
        },
        asm,
        // TODO per bundle state information so ASMs can be reused?
    ));
}
