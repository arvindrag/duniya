use bevy::{
    ecs::{component::Component, system::Res},
    image::TextureAtlasLayout,
    sprite::Sprite,
    transform::components::Transform,
};
use bevy::{
    platform::collections::HashMap, prelude::*,
};

use crate::base::{animation::*, sprite::*};
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum CharacterState {
    Idle,
    Walk,
    Run,
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum CharacterDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Clone)]
pub struct Character {
    pub state: CharacterState,
    pub direction: CharacterDirection,
}

impl Character {
    pub fn bundle(
        asset_server: Res<AssetServer>,
        texture_atlas_layouts: ResMut<
            Assets<TextureAtlasLayout>,
        >,
        sheet: SpriteSheet,
    ) -> (Character, Sprite, Transform, Animator)
    {
        let (sprite, transform, animator) = sheet
            .bundle(
                asset_server,
                texture_atlas_layouts,
            );
        (
            Self {
                state: CharacterState::Idle,
                direction:
                    CharacterDirection::Left,
            },
            sprite,
            transform,
            animator,
        )
    }
}
