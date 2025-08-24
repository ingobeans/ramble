mod types;
pub use types::*;

use crate::{player::Stats, projectiles::Projectile};

#[derive(Clone)]
pub struct Weapon {
    /// Seperate from the parent item's regular stats,
    /// as these only apply when item is held (not offhand).
    pub stats: Stats,
    /// Projectile fired
    pub projectile: Projectile,
}

#[derive(Clone)]
pub enum ItemType {
    Helmet,
    Chestplate,
    Held(Weapon),
    Talisman,
}
impl Default for ItemType {
    fn default() -> Self {
        Self::Helmet
    }
}

#[derive(Clone, Default)]
pub struct Item {
    pub name: &'static str,
    pub ty: ItemType,
    pub sprite_x: f32,
    pub sprite_y: f32,
    pub stats: Stats,
}
