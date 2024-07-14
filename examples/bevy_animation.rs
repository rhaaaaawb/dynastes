use bevy::{prelude::*, render::texture::ImageSamplerDescriptor};
use dynastes::{
    state_machine::StateID, states::index::IndexState, BevyASM, DynastesAnimationBundle,
    DynastesAnimationPlugin, MaybeBevyStateInstance,
};

const COMMON_MSPF: f64 = 1000. / 8.;

fn main() {
    env_logger::init();

    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin {
            default_sampler: ImageSamplerDescriptor::nearest(),
        }))
        .add_plugins(DynastesAnimationPlugin)
        .add_systems(Startup, setup_animations)
        .run();
}

fn setup_animations(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut state_machines: ResMut<Assets<BevyASM>>,
) {
    commands.spawn(Camera2dBundle::default());

    let path = "sprite-sheet.png";
    let tile_size = UVec2 { x: 128, y: 128 };
    let columns = 26;
    let rows = 2;
    let padding = None;
    let offset = None;

    let texture_handle = asset_server.load(path);

    let atlas_layout = TextureAtlasLayout::from_grid(tile_size, columns, rows, padding, offset);
    let atlas_layout_handle = atlas_layouts.add(atlas_layout);
    let texture_atlas = TextureAtlas {
        layout: atlas_layout_handle,
        index: 0,
    };

    walk_animation_with_fluidity(
        &mut commands,
        &mut state_machines,
        texture_atlas.clone(),
        texture_handle.clone(),
        (-60., 0., 0.).into(),
        None,
    );
    walk_animation_with_fluidity(
        &mut commands,
        &mut state_machines,
        texture_atlas.clone(),
        texture_handle.clone(),
        (60., 0., 0.).into(),
        Some(1. / 2.),
    );
}

fn walk_animation_with_fluidity(
    commands: &mut Commands,
    state_machines: &mut ResMut<Assets<BevyASM>>,
    texture_atlas: TextureAtlas,
    texture_handle: Handle<Image>,
    position: Vec3,
    fluidity: Option<f64>,
) {
    let walk_id: StateID = "walk".to_string().into();
    let idle_id: StateID = "idle".to_string().into();

    let walk_state: IndexState =
        IndexState::new(0, 9, COMMON_MSPF, Some(idle_id.clone()), None, fluidity);
    let idle_state: IndexState =
        IndexState::new(26, 51, COMMON_MSPF, Some(walk_id.clone()), None, fluidity);

    let mut asm = BevyASM::with_default(idle_id, idle_state);
    asm.add_states(vec![(walk_id, walk_state)]);

    let asm_handle = state_machines.add(asm);
    let scale = 4.;

    commands.spawn(DynastesAnimationBundle {
        state_machine: asm_handle,
        animation_state: MaybeBevyStateInstance::default(),
        sprite_bundle: SpriteBundle {
            texture: texture_handle,
            transform: Transform::from_translation(position).with_scale(Vec3::splat(scale)),
            ..Default::default()
        },
        texture_atlas,
    });
}
