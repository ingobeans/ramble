use std::{collections::HashMap, u16};

use macroquad::prelude::*;

use crate::{
    assets::Assets,
    particles::{self, *},
    player::Stats,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum DamageType {
    Slashing,
    Piercing,
    Fire,
    Unholy,
    Holy,
}
impl DamageType {
    pub fn to_text(&self) -> &'static str {
        match self {
            DamageType::Slashing => "slashing",
            DamageType::Piercing => "piercing",
            DamageType::Fire => "fire",
            DamageType::Unholy => "unholy",
            DamageType::Holy => "holy",
        }
    }
}

#[derive(Clone, Copy)]
pub enum DrawType {
    Sprite(f32, f32),
    Particle(Particle),
}
impl Default for DrawType {
    fn default() -> Self {
        Self::Sprite(0.0, 0.0)
    }
}

#[derive(Clone, Default)]
pub struct Projectile {
    pub pos: Vec2,
    pub origin: Vec2,
    pub direction: Vec2,
    pub speed: f32,
    pub drag: f32,
    pub draw_type: DrawType,
    pub player_owned: bool,
    pub boomerang: bool,
    pub life: u16,
    pub lifetime: u16,
    pub hit_enemies: Vec<usize>,
    pub parent_hit_enemies: Vec<usize>,
    pub stats: Option<Stats>,
    pub radius: f32,
}
impl Projectile {
    pub fn on_hit(&self) -> Vec<Projectile> {
        let mut new_projectiles = Vec::new();
        let Some(stats) = self.stats.clone() else {
            return new_projectiles;
        };

        for (ty, items) in stats.on_hit_effects.into_iter() {
            for (proj, damage) in items.into_iter() {
                if ty.is_none_or(|ty| stats.damage.get(&ty).is_some_and(|f| *f > 0.0)) {
                    let mut proj = proj.clone();
                    proj.stats = self.stats.clone();
                    if let Some(stats) = &mut proj.stats {
                        stats.damage = damage.clone();
                    }
                    proj.parent_hit_enemies = self.parent_hit_enemies.clone();
                    proj.parent_hit_enemies
                        .append(&mut self.hit_enemies.clone());

                    proj.pos = self.pos;
                    proj.origin = self.origin;
                    proj.direction = self.direction;
                    proj.player_owned = true;
                    new_projectiles.push(proj);
                }
            }
        }
        new_projectiles
    }
    pub fn draw(&self, assets: &Assets) {
        let x = self.pos.x.floor();
        let y = self.pos.y.floor();
        match &self.draw_type {
            DrawType::Sprite(sprite_x, sprite_y) => {
                let params = DrawTextureParams {
                    rotation: self.direction.to_angle(),
                    ..Default::default()
                };
                assets
                    .particles
                    .draw_sprite(x, y, *sprite_x, *sprite_y, Some(&params));
            }
            DrawType::Particle(particle) => {
                let ctx = ParticleContext {
                    pos: self.pos,
                    origin: self.origin,
                    life: self.life,
                };
                particle(&ctx, assets);
            }
        }
    }
}
pub const BASE_PROJECTILE: Projectile = Projectile {
    pos: Vec2::ZERO,
    origin: Vec2::ZERO,
    direction: Vec2::ZERO,
    speed: 0.0,
    drag: 0.0,
    draw_type: DrawType::Sprite(0.0, 0.0),
    life: 0,
    lifetime: 0,
    boomerang: false,
    stats: None,
    player_owned: false,
    radius: 6.0,
    hit_enemies: Vec::new(),
    parent_hit_enemies: Vec::new(),
};

pub fn acid_puddle() -> Projectile {
    Projectile {
        speed: 0.0,
        draw_type: DrawType::Particle(&particles::ACID_PUDDLE),
        lifetime: u16::MAX,
        radius: 12.0,
        ..BASE_PROJECTILE
    }
}

pub fn slimeball() -> Projectile {
    Projectile {
        speed: 0.8,
        draw_type: DrawType::Sprite(10.0, 0.0),
        lifetime: 120,
        ..BASE_PROJECTILE
    }
}
pub fn slash() -> Projectile {
    Projectile {
        speed: 5.0,
        drag: 0.15,
        draw_type: DrawType::Sprite(0.0, 0.0),
        lifetime: 20,
        ..BASE_PROJECTILE
    }
}
pub fn dark_slash() -> Projectile {
    Projectile {
        speed: 8.0,
        drag: 0.35,
        draw_type: DrawType::Sprite(9.0, 0.0),
        lifetime: 20,
        ..BASE_PROJECTILE
    }
}
pub fn fireball() -> Projectile {
    Projectile {
        speed: 2.0,
        draw_type: DrawType::Sprite(2.0, 0.0),
        lifetime: 140,
        ..BASE_PROJECTILE
    }
}
pub fn boomerang() -> Projectile {
    Projectile {
        speed: 3.0,
        draw_type: DrawType::Sprite(12.0, 0.0),
        lifetime: 140,
        boomerang: true,
        ..BASE_PROJECTILE
    }
}
pub fn arrow() -> Projectile {
    Projectile {
        speed: 7.0,
        draw_type: DrawType::Sprite(1.0, 0.0),
        lifetime: 80,
        ..BASE_PROJECTILE
    }
}
pub fn slow_arrow() -> Projectile {
    Projectile {
        speed: 3.0,
        draw_type: DrawType::Sprite(1.0, 0.0),
        lifetime: 160,
        ..BASE_PROJECTILE
    }
}
pub fn boxing_glove() -> Projectile {
    Projectile {
        speed: 4.0,
        drag: 0.1,
        draw_type: DrawType::Sprite(3.0, 0.0),
        lifetime: 20,
        ..BASE_PROJECTILE
    }
}
pub fn icicle() -> Projectile {
    Projectile {
        speed: 6.0,
        lifetime: 90,
        draw_type: DrawType::Sprite(4.0, 0.0),
        ..BASE_PROJECTILE
    }
}
pub fn power_orb() -> Projectile {
    Projectile {
        speed: 2.0,
        lifetime: 60,
        draw_type: DrawType::Sprite(5.0, 0.0),
        ..BASE_PROJECTILE
    }
}
pub fn blue_power_orb() -> Projectile {
    Projectile {
        speed: 1.5,
        lifetime: 90,
        draw_type: DrawType::Sprite(11.0, 0.0),
        ..BASE_PROJECTILE
    }
}
pub fn light_ray() -> Projectile {
    Projectile {
        speed: 16.0,
        lifetime: 6,
        draw_type: DrawType::Particle(&particles::LIGHT_RAY),
        ..BASE_PROJECTILE
    }
}
pub fn star_explosion() -> Projectile {
    Projectile {
        speed: 0.0,
        lifetime: 15,
        radius: 12.0,
        draw_type: DrawType::Particle(&particles::STAR_EXPLOSION),
        ..BASE_PROJECTILE
    }
}
pub fn star_bazooka() -> Projectile {
    Projectile {
        speed: 2.0,
        lifetime: 160,
        draw_type: DrawType::Sprite(6.0, 0.0),
        ..BASE_PROJECTILE
    }
}
pub fn fire() -> Projectile {
    Projectile {
        speed: 0.0,
        lifetime: 15,
        draw_type: DrawType::Particle(&particles::FIRE),
        ..BASE_PROJECTILE
    }
}
pub fn lance() -> Projectile {
    Projectile {
        speed: 12.0,
        drag: 0.25,
        draw_type: DrawType::Sprite(112.0 / 16.0, 0.0),
        lifetime: 20,
        ..BASE_PROJECTILE
    }
}
pub fn razor_dart() -> Projectile {
    Projectile {
        speed: 6.0,
        drag: 0.05,
        draw_type: DrawType::Sprite(128.0 / 16.0, 0.0),
        lifetime: 40,
        ..BASE_PROJECTILE
    }
}
