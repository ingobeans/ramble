mod types;
use std::{borrow::Cow, mem::discriminant};

pub use types::*;

use crate::{assets::Assets, player::Stats, projectiles::Projectile};

#[derive(Clone)]
pub struct Weapon {
    /// Projectile fired
    pub projectile: Projectile,
}

#[derive(Clone)]
pub struct Enchantment {
    name: &'static str,
    stats: Stats,
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
    pub internal_stats: Stats,
    pub enchantment: Option<Enchantment>,
}
impl Item {
    pub fn name(&self) -> Cow<'_, str> {
        if let Some(e) = &self.enchantment {
            Cow::Owned(format!("{} of {}", self.name, e.name))
        } else {
            Cow::Borrowed(self.name)
        }
    }
    pub fn stats(&self) -> Stats {
        let mut stats = self.internal_stats.clone();
        if let Some(e) = self.enchantment.clone() {
            stats.merge(&e.stats);
        }
        stats
    }
}
