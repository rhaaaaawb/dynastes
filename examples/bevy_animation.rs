use bevy::prelude::*;
use dynastes::{
    bevy::SpriteAnimationPlugin,
    state_machine::{AnimationStateMachine, StateID},
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

    let sprite_sheet: Handle<Image> = asset_server.load("sprite-sheet.png");
    let texture_atlas =
        TextureAtlas::from_grid(sprite_sheet, [128., 128.].into(), 26, 2, None, None);
    let texture_atlas_handle = sprites.add(texture_atlas);

    let walk_id: StateID = "walk".to_string().into();
    let idle_id: StateID = "idle".to_string().into();

    let walk_state: IndexState<TextureAtlasSprite> =
        IndexState::new(0, 9, 1000. / 15., Some(idle_id.clone()), false);
    let idle_state: IndexState<TextureAtlasSprite> =
        IndexState::new(26, 51, 1000. / 15., Some(walk_id.clone()), false);

    let mut asm = AnimationStateMachine::new(texture_atlas_handle.clone(), idle_id, idle_state);
    asm.add_states(vec![(walk_id, walk_state)]);

    commands.spawn((
        SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: texture_atlas_handle,
            ..Default::default()
        },
        asm,
    ));
}
