use std::fs;

use bevy::prelude::*;
use dynastes::{
    bevy::{BevyASM, BevyFrameSource, SpriteAnimationPlugin, TextureAtlasGridMetadata},
    state_machine::StateID,
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
) {
    commands.spawn(Camera2dBundle::default());

    let frame_soure = BevyFrameSource {
        path: "sprite-sheet.png".into(),
        metadata: TextureAtlasGridMetadata {
            tile_size: [128., 128.].into(),
            columns: 26,
            rows: 2,
            padding: None,
            offset: None,
        },
    };

    let texture_atlas_handle = sprites.add(frame_soure.make_texture_atlas(asset_server));

    let walk_id: StateID = "walk".to_string().into();
    let idle_id: StateID = "idle".to_string().into();

    let walk_state: IndexState<TextureAtlasSprite> =
        IndexState::new(0, 9, 1000. / 15., Some(idle_id.clone()), false);
    let idle_state: IndexState<TextureAtlasSprite> =
        IndexState::new(26, 51, 1000. / 15., Some(walk_id.clone()), false);

    let mut asm = BevyASM::new(frame_soure, idle_id, idle_state);
    asm.0.add_states(vec![(walk_id, walk_state)]);

    // I've left this here so it's easy to reset the serialized file
    // for whenever I happend to change the format
    let asm_str = ron::to_string(&asm).unwrap();
    let _ = fs::write("assets/state-machine.ron", asm_str);

    commands.spawn((
        SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: texture_atlas_handle,
            ..Default::default()
        },
        asm,
    ));
}
