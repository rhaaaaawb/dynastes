use std::fs;

use bevy::prelude::*;
use dynastes::bevy::{BevyASM, BevyStateInstance, SpriteAnimationPlugin};

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
    mut state_machines: ResMut<Assets<BevyASM>>,
) {
    commands.spawn(Camera2dBundle::default());

    // TODO: the next step is to somehow do this with AssetServer::load (?)
    let asm_str = fs::read_to_string("assets/state-machine.ron").unwrap();
    let asm: BevyASM = ron::from_str(&asm_str).unwrap();

    let texture_atlas_handle = sprites.add(asm.0.frame_source().make_texture_atlas(asset_server));
    let asm_handle = state_machines.add(asm);

    let instance_str = fs::read_to_string("assets/default_instance.ron").unwrap();
    let instance: BevyStateInstance = ron::from_str(&instance_str).unwrap();

    commands.spawn((
        SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: texture_atlas_handle,
            ..Default::default()
        },
        asm_handle,
        instance,
    ));
}
