use macroquad::prelude::*;
use std::sync::LazyLock;

use crate::{
    dungeon::World,
    enemy::{EnemyMovement, EnemyPhase, EnemyType, PhaseEndCondition, ProjectileFiring},
    projectiles,
};

pub static CRYPT: LazyLock<World> = LazyLock::new(|| World {
    background_color: Color::from_hex(0x180d2f),
    light: vec![
        // skeleton
        EnemyType {
            speed: 0.5,
            phases: vec![EnemyPhase {
                sprite_x: 2.0,
                sprite_y: 1.0,
                movement: EnemyMovement::Chase,
                firing: ProjectileFiring::None,
                end: PhaseEndCondition::None,
                frames: 2,
            }],
            max_health: 6.0,
        },
        // slime
        EnemyType {
            speed: 1.0,
            phases: vec![
                EnemyPhase {
                    sprite_x: 2.0,
                    sprite_y: 3.0,
                    movement: EnemyMovement::Chase,
                    firing: ProjectileFiring::None,
                    end: PhaseEndCondition::Frames(8),
                    frames: 4,
                },
                EnemyPhase {
                    sprite_x: 2.0,
                    sprite_y: 3.0,
                    movement: EnemyMovement::Still,
                    firing: ProjectileFiring::None,
                    end: PhaseEndCondition::Frames(8),
                    frames: 1,
                },
            ],
            max_health: 6.0,
        },
    ],
    heavy: vec![
        // sorcerer skeleton
        EnemyType {
            speed: 0.5,
            phases: vec![
                EnemyPhase {
                    sprite_x: 6.0,
                    sprite_y: 1.0,
                    movement: EnemyMovement::Chase,
                    firing: ProjectileFiring::None,
                    end: PhaseEndCondition::Frames(60),
                    frames: 2,
                },
                EnemyPhase {
                    sprite_x: 8.0,
                    sprite_y: 1.0,
                    movement: EnemyMovement::Still,
                    firing: ProjectileFiring::Forwards(projectiles::blue_power_orb(), 15),
                    end: PhaseEndCondition::Frames(15),
                    frames: 1,
                },
            ],
            max_health: 25.0,
        },
    ],
    ranged: vec![
        // skeleton archer
        EnemyType {
            speed: 0.25,
            phases: vec![EnemyPhase {
                sprite_x: 4.0,
                sprite_y: 1.0,
                movement: EnemyMovement::Wander(true),
                firing: ProjectileFiring::Forwards(projectiles::slow_arrow(), 35),
                end: PhaseEndCondition::None,
                frames: 2,
            }],
            max_health: 6.0,
        },
    ],
    other: vec![
        // skeleton with hammer
        EnemyType {
            speed: 0.45,
            phases: vec![
                EnemyPhase {
                    sprite_x: 9.0,
                    sprite_y: 1.0,
                    movement: EnemyMovement::Chase,
                    firing: ProjectileFiring::None,
                    end: PhaseEndCondition::PlayerDistance(48.0),
                    frames: 2,
                },
                EnemyPhase {
                    sprite_x: 11.0,
                    sprite_y: 1.0,
                    movement: EnemyMovement::Still,
                    firing: ProjectileFiring::Forwards(projectiles::hammer(), 70),
                    end: PhaseEndCondition::Frames(70),
                    frames: 1,
                },
            ],
            max_health: 20.0,
        },
    ],
    miniboss: vec![
        // skeleton-slime
        EnemyType {
            speed: 0.5,
            phases: vec![
                EnemyPhase {
                    sprite_x: 6.0,
                    sprite_y: 3.0,
                    movement: EnemyMovement::Wander(true),
                    firing: ProjectileFiring::Forwards(projectiles::slimeball(), 35),
                    end: PhaseEndCondition::HealthUnder(0.5),
                    frames: 2,
                },
                EnemyPhase {
                    sprite_x: 8.0,
                    sprite_y: 3.0,
                    movement: EnemyMovement::Chase,
                    firing: ProjectileFiring::None,
                    end: PhaseEndCondition::None,
                    frames: 2,
                },
            ],
            max_health: 100.0,
        },
    ],
});

pub static FOREST: LazyLock<World> = LazyLock::new(|| {
    World {
        background_color: Color::from_hex(0x1e090d),
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
                max_health: 4.0,
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
                max_health: 4.0,
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
                max_health: 15.0,
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
                max_health: 20.0,
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
            max_health: 6.0,
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
            max_health: 6.0,
        }],
        miniboss: vec![EnemyType {
            speed: 2.5,
            phases: vec![
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
                EnemyPhase {
                    sprite_x: 96.0 / 16.0,
                    sprite_y: 32.0 / 16.0,
                    movement: EnemyMovement::Still,
                    firing: ProjectileFiring::Around(projectiles::slash(), 100, 8),
                    end: PhaseEndCondition::Frames(60),
                    frames: 1,
                },
            ],
            max_health: 40.0,
        }],
    }
});
