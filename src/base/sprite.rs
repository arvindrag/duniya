use bevy::{
    asset::LoadState, image::TextureAtlasBuilder,
    prelude::*,
};

use crate::base::animation::Animator;
use crate::base::character::*;

#[derive(Clone)]
pub struct StateFrames {
    pub start: usize,
    pub num: usize,
}

/// One source sprite sheet: a grid of `cell`-sized
/// frames with one row per direction.
pub struct SheetSource {
    pub path: &'static str,
    pub state: CharacterState,
    /// Frames in each row, in `SpriteSheet::rows`
    /// order.
    pub frames: [u32; 4],
}

/// Sprite sheets that get packed into a single
/// atlas at runtime.
pub struct SpriteSheet {
    /// Width and height of one frame cell.
    pub cell: u32,
    pub scale: f32,
    /// Facing shown by each row of every sheet.
    pub rows: [CharacterDirection; 4],
    pub sheets: &'static [SheetSource],
}

/// Holds a sheet's images while they load; replaced
/// by `Sprite` and `Animator` once the atlas is built.
#[derive(Component)]
pub struct LoadingSheet {
    sheet: &'static SpriteSheet,
    images: Vec<Handle<Image>>,
}

impl SpriteSheet {
    pub fn bundle(
        &'static self,
        asset_server: &AssetServer,
    ) -> (LoadingSheet, Transform) {
        (
            LoadingSheet {
                sheet: self,
                images: self
                    .sheets
                    .iter()
                    .map(|source| {
                        asset_server.load(source.path)
                    })
                    .collect(),
            },
            Transform::from_scale(Vec3::splat(
                self.scale,
            )),
        )
    }
}

pub struct SpriteSheetPlugin;

impl Plugin for SpriteSheetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, build_atlases);
    }
}

/// Once all of a sheet's images are loaded, packs
/// them into one atlas and splits it into frames.
fn build_atlases(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut layouts: ResMut<
        Assets<TextureAtlasLayout>,
    >,
    query: Query<(Entity, &LoadingSheet)>,
) {
    for (entity, loading) in &query {
        if let Some(failed) =
            loading.images.iter().find(|image| {
                matches!(
                    asset_server.load_state(*image),
                    LoadState::Failed(_)
                )
            })
        {
            error!(
                "Failed to load sprite sheet {:?}",
                failed.path()
            );
            commands
                .entity(entity)
                .remove::<LoadingSheet>();
            continue;
        }
        let Some(sources) = loading
            .images
            .iter()
            .map(|image| images.get(image))
            .collect::<Option<Vec<_>>>()
        else {
            // Still loading.
            continue;
        };

        let mut builder =
            TextureAtlasBuilder::default();
        for source in sources {
            builder.add_texture(None, source);
        }
        let (packed, _, texture) =
            match builder.build() {
                Ok(built) => built,
                Err(err) => {
                    error!(
                        "Failed to pack sprite sheets: {err}"
                    );
                    commands
                        .entity(entity)
                        .remove::<LoadingSheet>();
                    continue;
                }
            };

        // Packed rects follow insertion order, so
        // they line up with `sheet.sheets`.
        let sheet = loading.sheet;
        let mut layout =
            TextureAtlasLayout::new_empty(packed.size);
        let mut actions = Vec::new();
        for (source, rect) in
            sheet.sheets.iter().zip(&packed.textures)
        {
            for (row, (direction, &frames)) in sheet
                .rows
                .iter()
                .zip(&source.frames)
                .enumerate()
            {
                let start = layout.len();
                for col in 0..frames {
                    let min = rect.min
                        + UVec2::new(col, row as u32)
                            * sheet.cell;
                    layout.add_texture(
                        URect::from_corners(
                            min,
                            min + UVec2::splat(
                                sheet.cell,
                            ),
                        ),
                    );
                }
                actions.push((
                    source.state.clone(),
                    direction.clone(),
                    start..layout.len(),
                ));
            }
        }

        commands
            .entity(entity)
            .remove::<LoadingSheet>()
            .insert((
                Sprite::from_atlas_image(
                    images.add(texture),
                    TextureAtlas {
                        layout: layouts.add(layout),
                        index: 0,
                    },
                ),
                Animator::new(actions),
            ));
    }
}
