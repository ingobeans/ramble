use hashmap_macro::hashmap;

use crate::{
    items::{Enchantment, Item, ItemType, Weapon},
    player::*,
    projectiles::{self, DamageType},
};

pub fn gift() -> Item {
    Item {
        name: "gift",
        ty: ItemType::Talisman,
        sprite_x: 1.0,
        sprite_y: 4.0,
        ..Default::default()
    }
}

pub fn get_enchantments() -> Vec<Enchantment> {
    vec![
        Enchantment {
            name: "burning",
            stats: Stats {
                damage: hashmap!(DamageType::Fire => 2.0),
                ..Default::default()
            },
            sprite_x: 0.0,
            sprite_y: 0.0,
        },
        Enchantment {
            name: "haste",
            stats: Stats {
                attack_delay_mod: -0.3,
                ..Default::default()
            },
            sprite_x: 1.0,
            sprite_y: 0.0,
        },
        Enchantment {
            name: "sluggishness",
            stats: Stats {
                attack_delay_mod: 0.3,
                ..Default::default()
            },
            sprite_x: 2.0,
            sprite_y: 0.0,
        },
        Enchantment {
            name: "light feet",
            stats: Stats {
                move_speed_mod: 0.3,
                ..Default::default()
            },
            sprite_x: 3.0,
            sprite_y: 0.0,
        },
        Enchantment {
            name: "holiness",
            stats: Stats {
                damage: hashmap!(DamageType::Holy => 2.0),
                ..Default::default()
            },
            sprite_x: 4.0,
            sprite_y: 0.0,
        },
        Enchantment {
            name: "hate",
            stats: Stats {
                damage: hashmap!(DamageType::Unholy => 2.0),
                damage_modifiers: hashmap!(DamageType::Holy => -1.0),
                ..Default::default()
            },
            sprite_x: 5.0,
            sprite_y: 0.0,
        },
    ]
}

pub fn get_items() -> Vec<Item> {
    vec![
        Item {
            name: "iron chestplate",
            ty: ItemType::Chestplate,
            internal_stats: Stats {
                max_lives: 2,
                lives: 2,
                move_speed_mod: -0.2,
                roll_delay_mod: 0.2,
                ..Default::default()
            },
            sprite_x: 1.0,
            sprite_y: 0.0,
            ..Default::default()
        },
        Item {
            name: "iron helmet",
            ty: ItemType::Helmet,
            internal_stats: Stats {
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
            internal_stats: Stats {
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
            internal_stats: Stats {
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
            name: "boomerang",
            internal_stats: Stats {
                attack_delay: 20.0,
                damage: hashmap!(DamageType::Piercing => 1.0),
                ..Default::default()
            },
            ty: ItemType::Held(Box::new(Weapon {
                projectile: projectiles::boomerang(),
            })),
            sprite_x: 13.0,
            sprite_y: 2.0,
            ..Default::default()
        },
        Item {
            name: "deathblade",
            internal_stats: Stats {
                attack_delay: 12.0,
                damage: hashmap!(DamageType::Unholy => 2.1),
                ..Default::default()
            },
            ty: ItemType::Held(Box::new(Weapon {
                projectile: projectiles::dark_slash(),
            })),
            sprite_x: 12.0,
            sprite_y: 2.0,
            ..Default::default()
        },
        Item {
            name: "dagger",
            ty: ItemType::Held(Box::new(Weapon {
                projectile: projectiles::slash(),
            })),
            internal_stats: Stats {
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
            internal_stats: Stats {
                attack_delay: 30.0,
                damage: hashmap!(DamageType::Slashing => 5.0),
                ..Default::default()
            },
            sprite_x: 4.0,
            sprite_y: 2.0,
            ..Default::default()
        },
        Item {
            name: "icicle wand",
            ty: ItemType::Held(Box::new(Weapon {
                projectile: projectiles::icicle(),
            })),
            internal_stats: Stats {
                attack_delay: 30.0,
                damage: hashmap!(DamageType::Piercing => 5.0),
                ..Default::default()
            },
            sprite_x: 5.0,
            sprite_y: 2.0,
            ..Default::default()
        },
        Item {
            name: "leather tunic",
            ty: ItemType::Chestplate,
            internal_stats: Stats {
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
            internal_stats: Stats {
                roll_delay_mod: -0.25,
                move_speed_mod: 0.25,
                ..Default::default()
            },
            sprite_x: 2.0,
            sprite_y: 1.0,
            ..Default::default()
        },
        Item {
            name: "fire wizards robes",
            ty: ItemType::Chestplate,
            internal_stats: Stats {
                damage_modifiers: hashmap!(DamageType::Fire => 0.25),
                ..Default::default()
            },
            sprite_x: 5.0,
            sprite_y: 0.0,
            ..Default::default()
        },
        Item {
            name: "chainmail",
            ty: ItemType::Chestplate,
            internal_stats: Stats {
                max_lives: 1,
                lives: 1,
                ..Default::default()
            },
            sprite_x: 6.0,
            sprite_y: 0.0,
            ..Default::default()
        },
        Item {
            name: "wizards robes",
            ty: ItemType::Chestplate,
            internal_stats: Stats {
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
            internal_stats: Stats {
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
            internal_stats: Stats {
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
            internal_stats: Stats {
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
            internal_stats: Stats {
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
            internal_stats: Stats {
                attack_delay: 70.0,
                on_hit_effects: hashmap!(None => vec![(projectiles::star_explosion(),hashmap!(DamageType::Holy => 5.0))]),
                ..Default::default()
            },
            sprite_x: 8.0,
            sprite_y: 2.0,
            ..Default::default()
        },
        Item {
            name: "heart pendant",
            ty: ItemType::Talisman,
            internal_stats: Stats {
                max_lives: 1,
                lives: 1,
                ..Default::default()
            },
            sprite_x: 1.0,
            sprite_y: 3.0,
            ..Default::default()
        },
        Item {
            name: "fire necklace",
            ty: ItemType::Talisman,
            internal_stats: Stats {
                on_hit_effects: hashmap![Some(DamageType::Fire) => vec![(projectiles::fire(),hashmap!(DamageType::Fire => 2.0))]],
                ..Default::default()
            },
            sprite_x: 4.0,
            sprite_y: 3.0,
            ..Default::default()
        },
        Item {
            name: "book of fireballs",
            ty: ItemType::Held(Box::new(Weapon {
                projectile: projectiles::fireball(),
            })),
            internal_stats: Stats {
                attack_delay: 30.0,
                damage: hashmap!(DamageType::Fire => 4.0),
                ..Default::default()
            },
            sprite_x: 9.0,
            sprite_y: 2.0,
            ..Default::default()
        },
        Item {
            name: "lance",
            ty: ItemType::Held(Box::new(Weapon {
                projectile: projectiles::lance(),
            })),
            internal_stats: Stats {
                attack_delay: 50.0,
                damage: hashmap!(DamageType::Piercing =>4.0),
                ..Default::default()
            },
            sprite_x: 10.0,
            sprite_y: 2.0,
            ..Default::default()
        },
        Item {
            name: "pendant of swiftness",
            ty: ItemType::Talisman,
            internal_stats: Stats {
                move_speed_mod: 0.2,
                roll_delay_mod: -0.2,
                ..Default::default()
            },
            sprite_x: 2.0,
            sprite_y: 3.0,
            ..Default::default()
        },
        Item {
            name: "toolbelt of rapid fire",
            ty: ItemType::Talisman,
            internal_stats: Stats {
                attack_delay_mod: -0.2,
                ..Default::default()
            },
            sprite_x: 6.0,
            sprite_y: 3.0,
            ..Default::default()
        },
        Item {
            name: "razor dart",
            ty: ItemType::Held(Box::new(Weapon {
                projectile: projectiles::razor_dart(),
            })),
            internal_stats: Stats {
                attack_delay: 5.0,
                damage: hashmap!(DamageType::Piercing => 0.5),
                ..Default::default()
            },
            sprite_x: 11.0,
            sprite_y: 2.0,
            ..Default::default()
        },
    ]
}
