// Support configuring Bevy lints within code.
#![cfg_attr(
    bevy_lint,
    feature(register_tool),
    register_tool(bevy)
)]
// Disable console on Windows for non-dev builds.
#![cfg_attr(
    not(feature = "dev"),
    windows_subsystem = "windows"
)]

use crate::{
    assets::PLAYER,
    base::{
        animation::SpriteAnimationPlugin,
        character::{Character, CharacterState},
    },
};
use bevy::prelude::*;

mod assets;
mod base;

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<
        Assets<TextureAtlasLayout>,
    >,
) {
    // Spawn a 2D camera to see the sprite
    commands.spawn(Character::bundle(
        asset_server,
        texture_atlas_layouts,
        PLAYER,
    ));
    commands.spawn(Camera2d);
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SpriteAnimationPlugin)
        .add_systems(Startup, startup)
        .run();
}
