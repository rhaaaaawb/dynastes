use core::marker::PhantomData;

use bevy::prelude::*;

use crate::state_machine::AnimationStateMachine;

pub struct SpriteAnimationPlugin<Sprite>(PhantomData<Sprite>);

impl<S> Default for SpriteAnimationPlugin<S> {
    fn default() -> Self {
        Self(PhantomData::default())
    }
}

impl<S> Plugin for SpriteAnimationPlugin<S>
where
    S: Send + Sync + crate::state_machine::Sprite + Component,
{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, AnimationStateMachine::<S>::animation_system);
    }
}
