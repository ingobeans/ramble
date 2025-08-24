use crate::enemy::{EnemyMovement, EnemyType};

pub static ENEMY_TYPES: &[EnemyType] = &[
    // archer
    EnemyType {
        sprite_x: 0.0,
        sprite_y: 1.0,
        speed: 0.5,
        movement: EnemyMovement::Wander(true),
        frames: 2,
        max_health: 5.0,
    },
    // bear
    EnemyType {
        sprite_x: 0.0,
        sprite_y: 2.0,
        speed: 1.0,
        movement: EnemyMovement::Chase,
        frames: 2,
        max_health: 5.0,
    },
    // bird
    EnemyType {
        sprite_x: 0.0,
        sprite_y: 3.0,
        speed: 1.0,
        movement: EnemyMovement::Wander(false),
        frames: 2,
        max_health: 2.0,
    },
];
