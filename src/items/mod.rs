mod types;
pub use types::*;

#[derive(Clone, Copy)]
pub enum ItemType {
    Helmet,
    Chestplate,
    Held,
    Talisman,
}

#[derive(Clone, Copy)]
pub struct Item {
    pub name: &'static str,
    pub ty: ItemType,
    pub sprite_x: f32,
    pub sprite_y: f32,
}
