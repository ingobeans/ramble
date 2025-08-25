use macroquad::prelude::*;

use crate::{assets::Assets, particles::*, player::Stats};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum DamageType {
    Slashing,
    Piercing,
    Fire,
    Ice,
    Unholy,
    Holy,
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
    pub direction: Vec2,
    pub speed: f32,
    pub drag: f32,
    pub draw_type: DrawType,
    pub chain: Vec<Projectile>,
    pub player_owned: bool,
    pub life: u16,
    pub lifetime: u16,
    pub stats: Option<Stats>,
}
impl Projectile {
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
                    life: self.life,
                };
                particle(&ctx, assets);
            }
        }
    }
}
pub const BASE_PROJECTILE: Projectile = Projectile {
    pos: Vec2::ZERO,
    direction: Vec2::ZERO,
    speed: 0.0,
    drag: 0.0,
    draw_type: DrawType::Sprite(0.0, 0.0),
    chain: Vec::new(),
    life: 0,
    lifetime: 0,
    stats: None,
    player_owned: false,
};

pub fn slash() -> Projectile {
    Projectile {
        speed: 5.0,
        drag: 0.15,
        draw_type: DrawType::Sprite(0.0, 0.0),
        lifetime: 20,
        ..BASE_PROJECTILE
    }
}

pub fn arrow() -> Projectile {
    Projectile {
        speed: 7.0,
        drag: 0.03,
        draw_type: DrawType::Sprite(1.0, 0.0),
        lifetime: 20,
        ..BASE_PROJECTILE
    }
}
