use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::{FromWorld, World},
    utils::BoxedFuture,
};

use crate::bevy::{bevy_serde::BevyASMSerde, BevyASM, BevyFrameSource};

/// Loads `BevyASM`s using a serialized `.asm` file
pub struct AsmLoader;

impl FromWorld for AsmLoader {
    fn from_world(_world: &mut World) -> Self {
        Self
    }
}

impl AssetLoader for AsmLoader {
    fn extensions(&self) -> &[&str] {
        &["asm"]
    }

    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let string = std::str::from_utf8(bytes)?;
            let asm_serde: BevyASMSerde = ron::from_str(string)?;

            let asm = BevyASM::with_context(asm_serde.clone(), load_context);

            load_context.set_default_asset(
                LoadedAsset::new(asm).with_dependency(asm_serde.frame_source.into()),
            );
            Ok(())
        })
    }
}

/// Loads `TextureAtlas`s using a serialized `.fs` file
pub struct FrameSourceLoader;

impl AssetLoader for FrameSourceLoader {
    fn extensions(&self) -> &[&str] {
        &["fs"]
    }

    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let string = std::str::from_utf8(bytes)?;
            let frame_source: BevyFrameSource = ron::from_str(string)?;
            let texture_atlas = frame_source.with_context(load_context);
            load_context.set_default_asset(
                LoadedAsset::new(texture_atlas).with_dependency(frame_source.path.into()),
            );
            Ok(())
        })
    }
}

impl FromWorld for FrameSourceLoader {
    fn from_world(_world: &mut World) -> Self {
        Self
    }
}
