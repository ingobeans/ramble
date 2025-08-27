use std::sync::LazyLock;

use crate::{
    dungeon::World,
    enemy::{EnemyMovement, EnemyPhase, EnemyType, PhaseEndCondition, ProjectileFiring},
    projectiles,
};

pub static WORLD_FOREST: LazyLock<World> = LazyLock::new(|| {
    World {
        light: vec![
            // bird
            EnemyType {
                speed: 1.0,
                phases: vec![EnemyPhase {
                    sprite_x: 0.0,
                    sprite_y: 3.0,
                    movement: EnemyMovement::Wander(false),
                    firing: ProjectileFiring::None,
                    end: PhaseEndCondition::None,
                    frames: 2,
                }],
                max_health: 2.0,
            },
            // mini hood
            EnemyType {
                speed: 0.5,
                phases: vec![EnemyPhase {
                    sprite_x: 0.0,
                    sprite_y: 4.0,
                    movement: EnemyMovement::Chase,
                    firing: ProjectileFiring::None,
                    end: PhaseEndCondition::None,
                    frames: 2,
                }],
                max_health: 2.0,
            },
        ],
        heavy: vec![
            // bear
            EnemyType {
                speed: 1.0,
                phases: vec![EnemyPhase {
                    sprite_x: 0.0,
                    sprite_y: 2.0,
                    movement: EnemyMovement::Chase,
                    firing: ProjectileFiring::None,
                    end: PhaseEndCondition::None,
                    frames: 2,
                }],
                max_health: 5.0,
            },
            // goblin knife
            EnemyType {
                speed: 0.5,
                phases: vec![EnemyPhase {
                    sprite_x: 4.0,
                    sprite_y: 2.0,
                    movement: EnemyMovement::Chase,
                    firing: ProjectileFiring::None,
                    end: PhaseEndCondition::None,
                    frames: 2,
                }],
                max_health: 10.0,
            },
        ],
        ranged: vec![EnemyType {
            // archer
            speed: 0.5,
            phases: vec![EnemyPhase {
                sprite_x: 0.0,
                sprite_y: 1.0,
                movement: EnemyMovement::Wander(true),
                firing: ProjectileFiring::Forwards(projectiles::slow_arrow(), 50),
                end: PhaseEndCondition::None,
                frames: 2,
            }],
            max_health: 5.0,
        }],
        other: vec![EnemyType {
            // red hooded fireball shooter
            speed: 0.0,
            phases: vec![EnemyPhase {
                sprite_x: 2.0,
                sprite_y: 2.0,
                movement: EnemyMovement::Chase,
                firing: ProjectileFiring::Forwards(projectiles::fireball(), 60),
                end: PhaseEndCondition::None,
                frames: 1,
            }],
            max_health: 5.0,
        }],
        miniboss: vec![EnemyType {
            speed: 2.0,
            phases: vec![
                EnemyPhase {
                    sprite_x: 96.0 / 16.0,
                    sprite_y: 32.0 / 16.0,
                    movement: EnemyMovement::Still,
                    firing: ProjectileFiring::None,
                    end: PhaseEndCondition::Frames(60),
                    frames: 1,
                },
                EnemyPhase {
                    sprite_x: 96.0 / 16.0,
                    sprite_y: 32.0 / 16.0,
                    movement: EnemyMovement::Chase,
                    firing: ProjectileFiring::None,
                    end: PhaseEndCondition::SingleFrame,
                    frames: 2,
                },
                EnemyPhase {
                    sprite_x: 96.0 / 16.0,
                    sprite_y: 32.0 / 16.0,
                    movement: EnemyMovement::Fowards,
                    firing: ProjectileFiring::None,
                    end: PhaseEndCondition::Collision,
                    frames: 2,
                },
            ],
            max_health: 40.0,
        }],
    }
});
