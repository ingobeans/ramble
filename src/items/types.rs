use hashmap_macro::hashmap;

use crate::{
    items::{Item, ItemType, Weapon},
    player::*,
    projectiles::{self, DamageType},
};
pub fn get_items() -> Vec<Item> {
    vec![
        Item {
            name: "iron chestplate",
            ty: ItemType::Chestplate,
            stats: Stats {
                max_lives: 1,
                lives: 1,
                speed_mod: -0.1,
                ..Default::default()
            },
            sprite_x: 0.0,
            sprite_y: 0.0,
            ..Default::default()
        },
        Item {
            name: "iron helmet",
            ty: ItemType::Helmet,
            stats: Stats {
                max_lives: 1,
                lives: 1,
                ..Default::default()
            },
            sprite_x: 0.0,
            sprite_y: 1.0,
            ..Default::default()
        },
        Item {
            name: "bow",
            ty: ItemType::Held(Weapon {
                stats: Stats {
                    attack_delay: 30.0,
                    damage: hashmap!(DamageType::Piercing => 1.0),
                    ..Default::default()
                },
                projectile: projectiles::arrow(),
            }),
            sprite_x: 0.0,
            sprite_y: 2.0,
            ..Default::default()
        },
        Item {
            name: "longsword",
            ty: ItemType::Held(Weapon {
                stats: Stats {
                    attack_delay: 10.0,
                    damage: hashmap!(DamageType::Slashing => 2.0),
                    ..Default::default()
                },
                projectile: projectiles::slash(),
            }),
            sprite_x: 1.0,
            sprite_y: 2.0,
            ..Default::default()
        },
        Item {
            name: "dagger",
            ty: ItemType::Held(Weapon {
                stats: Stats {
                    attack_delay: 10.0,
                    damage: hashmap!(DamageType::Slashing => 1.0),
                    ..Default::default()
                },
                projectile: projectiles::slash(),
            }),
            stats: Stats {
                speed_mod: 0.1,
                ..Default::default()
            },
            sprite_x: 2.0,
            sprite_y: 2.0,
            ..Default::default()
        },
    ]
}
