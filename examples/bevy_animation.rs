use bevy::prelude::*;
use dynastes::{
    bevy::{
        BevyASM, BevyFrameSource, DynastesAnimationBundle, MaybeBevyStateInstance,
        SpriteAnimationPlugin, TextureAtlasGridMetadata,
    },
    state_machine::StateID,
    states::index::IndexState,
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
    mut state_machines: ResMut<Assets<BevyASM>>,
) {
    commands.spawn(Camera2dBundle::default());

    let fs = BevyFrameSource {
        path: "sprite-sheet.png".into(),
        metadata: TextureAtlasGridMetadata {
            tile_size: [128., 128.].into(),
            columns: 26,
            rows: 2,
            padding: None,
            offset: None,
        },
    };

    let texture_handle = asset_server.load(fs.path.clone());
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        fs.metadata.tile_size,
        fs.metadata.columns,
        fs.metadata.rows,
        fs.metadata.padding,
        fs.metadata.offset,
    );
    let texture_atlas_handle = sprites.add(texture_atlas);

    let walk_id: StateID = "walk".to_string().into();
    let idle_id: StateID = "idle".to_string().into();

    let walk_state: IndexState<TextureAtlasSprite> =
        IndexState::new(0, 9, 1000. / 15., Some(idle_id.clone()));
    let idle_state: IndexState<TextureAtlasSprite> =
        IndexState::new(26, 51, 1000. / 15., Some(walk_id.clone()));

    let mut asm = BevyASM::new(texture_atlas_handle.clone(), idle_id, idle_state);
    asm.0.add_states(vec![(walk_id, walk_state)]);

    // let asm_str = ron::to_string(&asm.serialize_with_server(asset_server).unwrap()).unwrap();
    // let _ = fs::write("assets/state-machine.asm", asm_str);

    // let fs_str = ron::to_string(&fs).unwrap();
    // let _ = fs::write("assets/sprite-sheet.fs", fs_str);

    let asm_handle = state_machines.add(asm);

    commands.spawn(DynastesAnimationBundle {
        state_machine: asm_handle,
        animation_state: MaybeBevyStateInstance::default(),
        sprite_sheet: SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: texture_atlas_handle,
            ..Default::default()
        },
    });
}
