use bevy::prelude::*;

/// Current arrow-key direction, updated every
/// frame. `x` is -1 (left) to 1 (right), `y` is
/// -1 (down) to 1 (up). Zero when no key is held.
#[derive(Resource, Default, Debug)]
pub struct ArrowInput {
    pub direction: Vec2,
}

pub struct InputControllerPlugin;

impl Plugin for InputControllerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ArrowInput>()
            .add_systems(PreUpdate, read_arrows);
    }
}

fn read_arrows(
    keys: Res<ButtonInput<KeyCode>>,
    mut input: ResMut<ArrowInput>,
) {
    let mut direction = Vec2::ZERO;
    if keys.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keys.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if keys.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keys.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }
    input.direction = direction;
}
