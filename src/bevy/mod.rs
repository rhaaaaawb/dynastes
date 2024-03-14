use std::path::PathBuf;

use bevy::{
    asset::LoadContext,
    prelude::{AssetServer, Bundle, Component, Handle, Image, Res, Vec2},
    reflect::{TypePath, TypeUuid},
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
};
use serde::{Deserialize, Serialize};

use crate::{
    state_machine::{AnimationStateMachine, IndexSprite, Sprite, StateID, StateInstance},
    states::index::{IndexData, IndexState},
};

/// Serde helper structs for the bevy plugin
pub mod bevy_serde;
/// Asset loaders for the bevy plugin
pub mod loader;
mod plugin;

pub use plugin::SpriteAnimationPlugin;

use self::bevy_serde::BevyASMSerde;

#[derive(Bundle)]
/// A Bundle of the components needed to run an animation with Bevy ECS
pub struct DynastesAnimationBundle {
    /// The animation state machine
    pub state_machine: Handle<BevyASM>,
    /// The current state in `state_machine`
    pub animation_state: MaybeBevyStateInstance,
    /// The sprite sheet that the animation is across
    pub sprite_sheet: SpriteSheetBundle,
}

/// A convenience wrapper for the bevy monomorphization of the ASM
#[derive(Debug, Component, TypeUuid, TypePath)]
#[uuid = "74377e21-153d-4e30-9b5e-1b857a9ab807"]
pub struct BevyASM(
    pub  AnimationStateMachine<
        TextureAtlasSprite,
        IndexState<TextureAtlasSprite>,
        Handle<TextureAtlas>,
    >,
);

impl BevyASM {
    /// Creates a new Bevy ASM initialized with `default_id` and `default_state`
    pub fn new(
        frame_source: Handle<TextureAtlas>,
        default_id: StateID,
        default_state: IndexState<TextureAtlasSprite>,
    ) -> Self {
        BevyASM(AnimationStateMachine::with_default(
            frame_source,
            default_id,
            default_state,
        ))
    }

    /// Creates a new Bevy ASM initialized with `default_id` and `default_state`
    pub fn with_context<'a>(asm_serde: BevyASMSerde, load_context: &'a mut LoadContext) -> Self {
        let frame_source = load_context.get_handle::<_, TextureAtlas>(&asm_serde.frame_source);

        BevyASM(AnimationStateMachine::with_states(
            frame_source,
            asm_serde.default_id,
            asm_serde.states,
        ))
    }

    /// Creates a new instance from the default state
    pub fn default_instance(&self) -> BevyStateInstance {
        BevyStateInstance(self.0.default_instance())
    }

    /// Creates a new instance from the given state id if it exists
    pub fn new_instance(&self, instance_id: StateID) -> Option<BevyStateInstance> {
        self.0
            .new_instance(instance_id)
            .map(|i| BevyStateInstance(i))
    }

    /// Converts the Bevy-safe struct into a serializable struct with the help of the given AssetServer
    pub fn serialize_with_server(&self, server: Res<AssetServer>) -> Option<BevyASMSerde> {
        Some(BevyASMSerde {
            frame_source: server
                .get_handle_path(self.0.frame_source())?
                .path()
                .to_str()?
                .to_string(),
            default_id: self.0.default_id().to_owned(),
            states: self.0.states().to_owned(),
        })
    }
}

/// A convenience wrapper monomorphizing the `StateInstance` for the BevyASM
#[derive(Debug, Serialize, Deserialize, Component, TypePath)]
pub struct BevyStateInstance(
    pub StateInstance<IndexState<TextureAtlasSprite>, IndexData<TextureAtlasSprite>>,
);

/// A convenience wrapper for an optional `BevyStateInstance`
#[derive(Debug, Serialize, Deserialize, Component, TypePath, Default)]
pub struct MaybeBevyStateInstance(pub Option<BevyStateInstance>);

impl Sprite for TextureAtlasSprite {}

impl IndexSprite for TextureAtlasSprite {
    fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    fn get_index(&self) -> usize {
        self.index
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TypePath, TypeUuid)]
#[uuid = "73b8df5e-c12d-4830-83c8-faec6ee4e18d"]
/// The source of the `BevyASM`'s frame
pub struct BevyFrameSource {
    /// The asset path to the ASM's sprite sheet
    pub path: PathBuf,
    /// Metadata for constructing the texture atlas from the sprite sheet
    pub metadata: TextureAtlasGridMetadata,
}

impl BevyFrameSource {
    /// Loads the referenced sprite sheet and converts it to a texture atlas
    pub fn with_context<'a>(&self, load_context: &'a mut LoadContext) -> TextureAtlas {
        let handle = load_context.get_handle::<_, Image>(self.path.to_str().unwrap());
        TextureAtlas::from_grid(
            handle,
            self.metadata.tile_size,
            self.metadata.columns,
            self.metadata.rows,
            self.metadata.padding,
            self.metadata.offset,
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Metadata needed for constructing a grided texture atlas from a sprite sheet
pub struct TextureAtlasGridMetadata {
    /// The size of each sprite in pixels
    pub tile_size: Vec2,
    /// The number of columns in the sprite sheet
    pub columns: usize,
    /// The number of rows in the sprite sheet
    pub rows: usize,
    /// Separation between each sprite
    pub padding: Option<Vec2>,
    /// Where the grid starts relative to the top left corner
    pub offset: Option<Vec2>,
}
