use bevy::{
    ecs::component::Component,
    transform::components::Transform,
};
use bevy::{
    platform::collections::HashMap, prelude::*,
};

use crate::base::{
    input::ArrowInput, movement::*, sprite::*,
};

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum CharacterState {
    Idle,
    Walk,
    Run,
}

#[derive(Component, Clone)]
pub struct Character {
    pub state: CharacterState,
}

/// Marks the character steered by the arrow keys.
#[derive(Component)]
pub struct Player;

impl Character {
    pub fn bundle(
        asset_server: &AssetServer,
        sheet: &'static SpriteSheet,
    ) -> (
        Character,
        LoadingSheet,
        Transform,
        Movable,
    ) {
        let (loading, transform) =
            sheet.bundle(asset_server);
        (
            Self {
                state: CharacterState::Idle,
            },
            loading,
            transform,
            Movable {
                direction: Direction::Down,
                velocity: Vec2::ZERO,
                max_speed: 200.0,
                run_speed: 120.0,
                acceleration: 300.0,
                state: MoveState::STATIONARY,
            },
        )
    }
}

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (steer_player).chain(),
        );
    }
}

/// Accelerates the player toward the arrow-key
/// direction, or decelerates to a stop.
fn steer_player(
    time: Res<Time>,
    input: Res<ArrowInput>,
    mut query: Query<
        (&mut Character, &mut Movable),
        With<Player>,
    >,
) {
    for (mut character, mut movable) in &mut query
    {
        let target = input
            .direction
            .normalize_or_zero()
            * movable.max_speed;
        let step =
            movable.acceleration * time.delta_secs();
        movable.velocity = movable
            .velocity
            .move_towards(target, step);

        character.state = match movable.state {
            MoveState::STATIONARY => {
                CharacterState::Idle
            }
            MoveState::WALKING => {
                CharacterState::Walk
            }
            MoveState::RUNNING => {
                CharacterState::Run
            }
        };
    }
}
