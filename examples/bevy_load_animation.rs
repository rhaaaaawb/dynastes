use bevy::prelude::*;
use dynastes::bevy::{
    BevyASM, DynastesAnimationBundle, MaybeBevyStateInstance, SpriteAnimationPlugin,
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

fn setup_animations(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    let asm: Handle<BevyASM> = asset_server.load("state-machine.asm");

    let texture_atlas = asset_server.load("sprite-sheet.fs");

    commands.spawn(DynastesAnimationBundle {
        state_machine: asm,
        animation_state: MaybeBevyStateInstance::default(),
        sprite_sheet: SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas,
            ..Default::default()
        },
    });
}
