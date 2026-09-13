use std::ops::Range;

use bevy::{
    platform::collections::HashMap, prelude::*,
};

use crate::base::{character::*, sprite::*};

#[derive(Component)]
pub struct Animator {
    timer: Timer,
    actions: HashMap<
        (CharacterState, CharacterDirection),
        Range<usize>,
    >,
}

impl Animator {
    pub fn new(
        actions: Vec<(
            CharacterState,
            CharacterDirection,
            Range<usize>,
        )>,
    ) -> Self {
        let mut actions_map = HashMap::new();
        for (state, direction, frames) in actions
        {
            actions_map
                .insert((state, direction), frames);
        }
        Self {
            timer: Timer::from_seconds(
                0.1,
                TimerMode::Repeating,
            ),
            actions: actions_map,
        }
    }
}

pub struct SpriteAnimationPlugin;

impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_sprite);
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &mut Animator,
        &mut Sprite,
        &Character,
    )>,
) {
    for (mut config, mut sprite, character) in
        &mut query
    {
        // Tick the timer based on real time elapsed
        config.timer.tick(time.delta());

        // If the timer completed its duration, advance the frame index
        // (skipped if the sheet has no frames for
        // this state/direction, e.g. Walk)
        if config.timer.just_finished()
            && let Some(atlas) =
                &mut sprite.texture_atlas
            && let Some(range) =
                config.actions.get(&(
                    character.state.clone(),
                    character.direction.clone(),
                ))
        {
            // Advance within the action's range, wrapping to its start. If the
            // index is outside the range (state/direction just changed), restart.
            atlas.index = if range
                .contains(&atlas.index)
                && atlas.index + 1 < range.end
            {
                atlas.index + 1
            } else {
                range.start
            };
        }
    }
}
