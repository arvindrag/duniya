use crate::base::{
    character::{
        CharacterDirection, CharacterState,
    },
    sprite::SpriteSheet,
};

// /// Index of each sheet in `PLAYER_SHEETS`.
// pub const PLAYER_IDLE: usize = 0;
// pub const PLAYER_RUN: usize = 1;

// pub const PLAYER_SHEETS: [SpriteSheet; 2] = [
//     SpriteSheet {
//         path: "images/Swordsman_lvl1_Idle_with_shadow.png",
//         rows: 4,
//         cols: 12,
//         scale: 2.0,
//     },
//     SpriteSheet {
//         path: "images/Swordsman_lvl1_Run_with_shadow.png",
//         rows: 4,
//         cols: 8,
//         scale: 2.0,
//     },
// ];

pub const PLAYER: SpriteSheet = SpriteSheet {
    path: "images/atlas_0.png",
    rows: 4,
    cols: 8,
    scale: 2.0,
    actions: [
        (
            CharacterState::Idle,
            CharacterDirection::Up,
            4..8,
        ),
        (
            CharacterState::Idle,
            CharacterDirection::Down,
            0..4,
        ),
        (
            CharacterState::Idle,
            CharacterDirection::Left,
            5..8,
        ),
        (
            CharacterState::Idle,
            CharacterDirection::Right,
            9..16,
        ),
        (
            CharacterState::Run,
            CharacterDirection::Up,
            10..19,
        ),
        (
            CharacterState::Run,
            CharacterDirection::Down,
            8..16,
        ),
        (
            CharacterState::Run,
            CharacterDirection::Left,
            8..16,
        ),
        (
            CharacterState::Run,
            CharacterDirection::Right,
            8..16,
        ),
    ],
};
