use bevy::prelude::*;
use dynastes::{
    bevy::SpriteAnimationPlugin, state_machine::AnimationStateMachine, states::IndexState,
};

fn main() {
    env_logger::init();

    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin {
            default_sampler: bevy::render::texture::ImageSampler::nearest_descriptor(),
        }))
        // add the plugin to our game, the 10 is the max number of nodes in a single chain
        // this provents the app getting stuck in a loop
        .add_plugins(SpriteAnimationPlugin::<TextureAtlasSprite>::default())
        .add_systems(Startup, setup_animations)
        .run()
}

fn setup_animations(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut sprites: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default());

    let sprite_sheet: Handle<Image> = asset_server.load("RH-run-front.png");
    let texture_atlas =
        TextureAtlas::from_grid(sprite_sheet, [128., 128.].into(), 10, 1, None, None);
    let texture_atlas_handle = sprites.add(texture_atlas);

    let walk_state: IndexState<TextureAtlasSprite> =
        IndexState::new(0, 9, 1000. / 15., None, false);

    let asm = AnimationStateMachine::new("walk".to_string().into(), Box::new(walk_state));

    commands.spawn((
        SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: texture_atlas_handle,
            ..Default::default()
        },
        asm,
    ));
}
