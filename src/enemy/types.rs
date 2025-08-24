use crate::enemy::{EnemyMovement, EnemyType};

pub static ENEMY_TYPES: &[EnemyType] = &[
    // archer
    EnemyType {
        sprite_x: 0.0,
        sprite_y: 1.0,
        speed: 0.5,
        movement: EnemyMovement::Wander,
        frames: 1,
    },
    // bear
    EnemyType {
        sprite_x: 0.0,
        sprite_y: 2.0,
        speed: 1.0,
        movement: EnemyMovement::Chase,
        frames: 2,
    },
    // bird
    EnemyType {
        sprite_x: 0.0,
        sprite_y: 3.0,
        speed: 1.0,
        movement: EnemyMovement::Wander,
        frames: 2,
    },
];
