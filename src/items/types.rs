use crate::items::{Item, ItemType};

pub static ITEMS: &[Item] = &[
    Item {
        name: "iron chestplate",
        ty: ItemType::Chestplate,
        sprite_x: 0.0,
        sprite_y: 0.0,
    },
    Item {
        name: "iron helmet",
        ty: ItemType::Helmet,
        sprite_x: 0.0,
        sprite_y: 1.0,
    },
    Item {
        name: "bow",
        ty: ItemType::Held,
        sprite_x: 0.0,
        sprite_y: 2.0,
    },
    Item {
        name: "longsword",
        ty: ItemType::Held,
        sprite_x: 1.0,
        sprite_y: 2.0,
    },
];
