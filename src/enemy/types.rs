use std::sync::LazyLock;

use crate::{
    enemy::{EnemyMovement, EnemyType, ProjectileFiring},
    projectiles,
};

pub static ENEMY_TYPES: LazyLock<Vec<EnemyType>> = LazyLock::new(|| {
    vec![
        // archer
        EnemyType {
            sprite_x: 0.0,
            sprite_y: 1.0,
            speed: 0.5,
            movement: EnemyMovement::Wander(true),
            projectile_firing: ProjectileFiring::TowardsPlayer(projectiles::arrow(), 30),
            frames: 2,
            max_health: 5.0,
        },
        // bear
        EnemyType {
            sprite_x: 0.0,
            sprite_y: 2.0,
            speed: 1.0,
            movement: EnemyMovement::Chase,
            projectile_firing: ProjectileFiring::None,
            frames: 2,
            max_health: 5.0,
        },
        // bird
        EnemyType {
            sprite_x: 0.0,
            sprite_y: 3.0,
            speed: 1.0,
            movement: EnemyMovement::Wander(false),
            projectile_firing: ProjectileFiring::None,
            frames: 2,
            max_health: 2.0,
        },
    ]
});
