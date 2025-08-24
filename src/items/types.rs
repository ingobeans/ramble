use crate::{
    items::{Item, ItemType, Weapon},
    player::*,
    projectiles,
};

const DEFAULT_ITEM: Item = Item {
    name: "",
    ty: ItemType::Helmet,
    sprite_x: 0.0,
    sprite_y: 0.0,
    stats: DEFAULT_STATS,
};

pub const ITEMS: &[Item] = &[
    Item {
        name: "iron chestplate",
        ty: ItemType::Chestplate,
        sprite_x: 0.0,
        sprite_y: 0.0,
        ..DEFAULT_ITEM
    },
    Item {
        name: "iron helmet",
        ty: ItemType::Helmet,
        sprite_x: 0.0,
        sprite_y: 1.0,
        ..DEFAULT_ITEM
    },
    Item {
        name: "bow",
        ty: ItemType::Held(Weapon {
            stats: Stats { ..DEFAULT_STATS },
            projectile: projectiles::SLASH,
        }),
        sprite_x: 0.0,
        sprite_y: 2.0,
        ..DEFAULT_ITEM
    },
    Item {
        name: "longsword",
        ty: ItemType::Held(Weapon {
            stats: Stats {
                attack_delay: 30.0,
                ..DEFAULT_STATS
            },
            projectile: projectiles::SLASH,
        }),
        sprite_x: 1.0,
        sprite_y: 2.0,
        ..DEFAULT_ITEM
    },
];
