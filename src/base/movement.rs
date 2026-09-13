use bevy::prelude::*;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub enum MoveState {
    STATIONARY,
    RUNNING,
    WALKING,
}

#[derive(Component)]
pub struct Movable {
    pub direction: Direction,
    pub velocity: Vec2,
    pub max_speed: f32,
    pub run_speed: f32,
    pub acceleration: f32,
    pub state: MoveState,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement);
    }
}

/// Moves characters by their velocity and derives
/// state and facing from it.
fn movement(
    time: Res<Time>,
    mut query: Query<(
        &mut Movable,
        &mut Transform,
    )>,
) {
    for (mut movable, mut transform) in &mut query
    {
        let velocity = movable.velocity;
        let speed = velocity.length();
        if speed == 0.0 {
            movable.state = MoveState::STATIONARY;
            return;
        }
        transform.translation += velocity
            .extend(0.0)
            * time.delta_secs();
        movable.state =
            if speed > movable.run_speed {
                MoveState::RUNNING
            } else {
                MoveState::WALKING
            };
        movable.direction = if velocity.x.abs()
            > velocity.y.abs()
        {
            if velocity.x > 0.0 {
                Direction::Right
            } else {
                Direction::Left
            }
        } else if velocity.y > 0.0 {
            Direction::Up
        } else {
            Direction::Down
        };
    }
}
