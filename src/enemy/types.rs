use std::sync::LazyLock;

use crate::{
    dungeon::World,
    enemy::{EnemyMovement, EnemyType, ProjectileFiring},
    projectiles,
};

pub static WORLD_FOREST: LazyLock<World> = LazyLock::new(|| {
    World {
        light: vec![
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
            // mini hood
            EnemyType {
                sprite_x: 0.0,
                sprite_y: 4.0,
                speed: 0.5,
                movement: EnemyMovement::Chase,
                projectile_firing: ProjectileFiring::None,
                frames: 2,
                max_health: 2.0,
            },
        ],
        heavy: vec![
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
            // goblin knife
            EnemyType {
                sprite_x: 4.0,
                sprite_y: 2.0,
                speed: 0.5,
                movement: EnemyMovement::Chase,
                projectile_firing: ProjectileFiring::None,
                frames: 2,
                max_health: 5.0,
            },
        ],
        ranged: vec![EnemyType {
            sprite_x: 0.0,
            sprite_y: 1.0,
            speed: 0.5,
            movement: EnemyMovement::Wander(true),
            projectile_firing: ProjectileFiring::Forwards(projectiles::slow_arrow(), 50),
            frames: 2,
            max_health: 5.0,
        }],
        other: vec![EnemyType {
            sprite_x: 2.0,
            sprite_y: 2.0,
            speed: 0.0,
            movement: EnemyMovement::Chase,
            projectile_firing: ProjectileFiring::Forwards(projectiles::fireball(), 60),
            frames: 1,
            max_health: 5.0,
        }],
        // archer
    }
});
