use bevy::{
    ecs::{component::Component, system::Res},
    image::TextureAtlasLayout,
    sprite::Sprite,
    transform::components::Transform,
};
use bevy::{
    platform::collections::HashMap, prelude::*,
};

use crate::base::{
    animation::*, input::ArrowInput, sprite::*,
};

/// Top speed, in pixels per second.
pub const MAX_SPEED: f32 = 200.0;
/// Speeds above this play the Run animation.
pub const RUN_SPEED: f32 = 120.0;
/// How fast velocity changes, in pixels/sec².
pub const ACCELERATION: f32 = 600.0;

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
    pub velocity: Vec2,
}

/// Marks the character steered by the arrow keys.
#[derive(Component)]
pub struct Player;

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
                velocity: Vec2::ZERO,
            },
            sprite,
            transform,
            animator,
        )
    }
}

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (steer_player, move_characters)
                .chain(),
        );
    }
}

/// Accelerates the player toward the arrow-key
/// direction, or decelerates to a stop.
fn steer_player(
    time: Res<Time>,
    input: Res<ArrowInput>,
    mut query: Query<
        &mut Character,
        With<Player>,
    >,
) {
    let target =
        input.direction.normalize_or_zero()
            * MAX_SPEED;
    let step = ACCELERATION * time.delta_secs();
    for mut character in &mut query {
        character.velocity = character
            .velocity
            .move_towards(target, step);
    }
}

/// Moves characters by their velocity and derives
/// state and facing from it.
fn move_characters(
    time: Res<Time>,
    mut query: Query<(
        &mut Character,
        &mut Transform,
    )>,
) {
    for (mut character, mut transform) in
        &mut query
    {
        let velocity = character.velocity;
        transform.translation += velocity
            .extend(0.0)
            * time.delta_secs();

        let speed = velocity.length();
        character.state = if speed == 0.0 {
            CharacterState::Idle
        } else if speed > RUN_SPEED {
            CharacterState::Run
        } else {
            CharacterState::Walk
        };

        // Face the dominant axis; keep the last
        // facing when stopped.
        if speed > 0.0 {
            character.direction =
                if velocity.x.abs()
                    > velocity.y.abs()
                {
                    if velocity.x > 0.0 {
                        CharacterDirection::Right
                    } else {
                        CharacterDirection::Left
                    }
                } else if velocity.y > 0.0 {
                    CharacterDirection::Up
                } else {
                    CharacterDirection::Down
                };
        }
    }
}
