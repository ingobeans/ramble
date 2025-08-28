mod types;
use std::mem::discriminant;

pub use types::*;

use crate::{assets::Assets, player::Stats, projectiles::Projectile};

#[derive(Clone)]
pub struct Weapon {
    /// Projectile fired
    pub projectile: Projectile,
}

#[derive(Clone)]
pub enum ItemType {
    Helmet,
    Chestplate,
    Held(Box<Weapon>),
    Talisman,
}
impl ItemType {
    pub fn draw_icon(&self, x: f32, y: f32, assets: &Assets) {
        let ty = match self {
            ItemType::Chestplate => 0.0,
            ItemType::Helmet => 1.0,
            ItemType::Held(_) => 2.0,
            ItemType::Talisman => 3.0,
        };
        assets.items.draw_sprite(x, y, 0.0, ty, None);
    }
    pub fn name(&self) -> &'static str {
        match self {
            ItemType::Talisman => "talisman",
            ItemType::Chestplate => "chestplate",
            ItemType::Helmet => "helmet",
            ItemType::Held(_) => "weapon",
        }
    }
}
impl PartialEq for ItemType {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
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
