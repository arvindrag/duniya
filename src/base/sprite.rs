use std::ops::Range;

use bevy::{
    platform::collections::HashMap, prelude::*,
};

use crate::base::animation::Animator;
use crate::base::character::*;

#[derive(Clone)]
pub struct StateFrames {
    pub start: usize,
    pub num: usize,
}

pub struct SpriteSheet {
    pub path: &'static str,
    pub rows: u32,
    pub cols: u32,
    pub scale: f32,
    pub actions: [(
        CharacterState,
        CharacterDirection,
        Range<usize>,
    ); 8],
}

impl SpriteSheet {
    pub fn bundle(
        self,
        asset_server: Res<AssetServer>,
        mut texture_atlas_layouts: ResMut<
            Assets<TextureAtlasLayout>,
        >,
    ) -> (Sprite, Transform, Animator) {
        let texture =
            asset_server.load(self.path);

        // Define grid: tile size (width, height), columns, rows
        let layout =
            TextureAtlasLayout::from_grid(
                UVec2::splat(64),
                self.cols,
                self.rows,
                None,
                None,
            );
        let atlas_layout_handle =
            texture_atlas_layouts.add(layout);

        (
            Sprite {
                image: texture,
                texture_atlas: Some(
                    TextureAtlas {
                        layout:
                            atlas_layout_handle,
                        index: 0, // Start at frame 0
                    },
                ),
                ..default()
            },
            Transform::from_scale(Vec3::splat(
                self.scale,
            )),
            Animator::new(&self.actions),
        )
    }
}
