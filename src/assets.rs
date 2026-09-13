use crate::base::{
    character::{
        CharacterDirection, CharacterState,
    },
    sprite::{SheetSource, SpriteSheet},
};

pub static PLAYER: SpriteSheet = SpriteSheet {
    cell: 64,
    scale: 2.0,
    rows: [
        CharacterDirection::Down,
        CharacterDirection::Left,
        CharacterDirection::Right,
        CharacterDirection::Up,
    ],
    sheets: &[
        SheetSource {
            path: "images/Swordsman1/Swordsman_lvl1_Idle_with_shadow.png",
            state: CharacterState::Idle,
            frames: [12, 12, 12, 4],
        },
        SheetSource {
            path: "images/Swordsman1/Swordsman_lvl1_Run_with_shadow.png",
            state: CharacterState::Run,
            frames: [8; 4],
        },
        SheetSource {
            path: "images/Swordsman1/Swordsman_lvl1_Walk_with_shadow.png",
            state: CharacterState::Walk,
            frames: [8; 4],
        },
    ],
};
