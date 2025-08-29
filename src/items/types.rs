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
                move_speed_mod: -0.1,
                ..Default::default()
            },
            sprite_x: 1.0,
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
            sprite_x: 1.0,
            sprite_y: 1.0,
            ..Default::default()
        },
        Item {
            name: "bow",
            stats: Stats {
                attack_delay: 30.0,
                damage: hashmap!(DamageType::Piercing => 1.0),
                ..Default::default()
            },
            ty: ItemType::Held(Box::new(Weapon {
                projectile: projectiles::arrow(),
            })),
            sprite_x: 1.0,
            sprite_y: 2.0,
            ..Default::default()
        },
        Item {
            name: "longsword",
            stats: Stats {
                attack_delay: 10.0,
                damage: hashmap!(DamageType::Slashing => 2.0),
                ..Default::default()
            },
            ty: ItemType::Held(Box::new(Weapon {
                projectile: projectiles::slash(),
            })),
            sprite_x: 2.0,
            sprite_y: 2.0,
            ..Default::default()
        },
        Item {
            name: "dagger",
            ty: ItemType::Held(Box::new(Weapon {
                projectile: projectiles::slash(),
            })),
            stats: Stats {
                attack_delay: 10.0,
                damage: hashmap!(DamageType::Slashing => 1.0),
                move_speed_mod: 0.1,
                ..Default::default()
            },
            sprite_x: 3.0,
            sprite_y: 2.0,
            ..Default::default()
        },
        Item {
            name: "boxing gloves",
            ty: ItemType::Held(Box::new(Weapon {
                projectile: projectiles::boxing_glove(),
            })),
            stats: Stats {
                attack_delay: 30.0,
                damage: hashmap!(DamageType::Slashing => 5.0),
                ..Default::default()
            },
            sprite_x: 4.0,
            sprite_y: 2.0,
            ..Default::default()
        },
        Item {
            name: "ice wand",
            ty: ItemType::Held(Box::new(Weapon {
                projectile: projectiles::icicle(),
            })),
            stats: Stats {
                attack_delay: 30.0,
                damage: hashmap!(DamageType::Ice => 5.0),
                ..Default::default()
            },
            sprite_x: 5.0,
            sprite_y: 2.0,
            ..Default::default()
        },
        Item {
            name: "leather tunic",
            ty: ItemType::Chestplate,
            stats: Stats {
                roll_delay_mod: -0.2,
                damage_modifiers: hashmap!(DamageType::Piercing => 0.05),
                ..Default::default()
            },
            sprite_x: 2.0,
            sprite_y: 0.0,
            ..Default::default()
        },
        Item {
            name: "archers hood",
            ty: ItemType::Helmet,
            stats: Stats {
                roll_delay_mod: -0.25,
                move_speed_mod: 0.25,
                ..Default::default()
            },
            sprite_x: 2.0,
            sprite_y: 1.0,
            ..Default::default()
        },
        Item {
            name: "wizards robes",
            ty: ItemType::Chestplate,
            stats: Stats {
                damage_modifiers: hashmap!(DamageType::Holy => 0.2, DamageType::Unholy => -0.2),
                ..Default::default()
            },
            sprite_x: 3.0,
            sprite_y: 0.0,
            ..Default::default()
        },
        Item {
            name: "wizards hat",
            ty: ItemType::Helmet,
            stats: Stats {
                damage_modifiers: hashmap!(DamageType::Holy => 0.2, DamageType::Unholy => -0.2),
                ..Default::default()
            },
            sprite_x: 3.0,
            sprite_y: 1.0,
            ..Default::default()
        },
        Item {
            name: "cobalt chestplate",
            ty: ItemType::Chestplate,
            stats: Stats {
                max_lives: 1,
                lives: 1,
                damage_modifiers: hashmap!(DamageType::Unholy => 0.2),
                ..Default::default()
            },
            sprite_x: 4.0,
            sprite_y: 0.0,
            ..Default::default()
        },
        Item {
            name: "power orb",
            ty: ItemType::Held(Box::new(Weapon {
                projectile: projectiles::power_orb(),
            })),
            stats: Stats {
                attack_delay: 30.0,
                damage: hashmap!(DamageType::Unholy => 4.0),
                ..Default::default()
            },
            sprite_x: 6.0,
            sprite_y: 2.0,
            ..Default::default()
        },
        Item {
            name: "light ray",
            ty: ItemType::Held(Box::new(Weapon {
                projectile: projectiles::light_ray(),
            })),
            stats: Stats {
                attack_delay: 15.0,
                damage: hashmap!(DamageType::Holy => 2.0),
                ..Default::default()
            },
            sprite_x: 7.0,
            sprite_y: 2.0,
            ..Default::default()
        },
        Item {
            name: "star bazooka",
            ty: ItemType::Held(Box::new(Weapon {
                projectile: projectiles::star_bazooka(),
            })),
            stats: Stats {
                attack_delay: 70.0,
                damage: hashmap!(DamageType::Holy => 5.0),
                ..Default::default()
            },
            sprite_x: 8.0,
            sprite_y: 2.0,
            ..Default::default()
        },
    ]
}
