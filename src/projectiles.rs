use macroquad::prelude::*;

use crate::{assets::Assets, particles::*};

#[derive(Clone, Copy)]
pub enum DrawType {
    Sprite(f32, f32),
    Particle(Particle),
}

#[derive(Clone)]
pub struct Projectile {
    pub pos: Vec2,
    pub direction: Vec2,
    pub speed: f32,
    pub drag: f32,
    pub draw_type: DrawType,
    pub chain: Vec<Projectile>,
    pub life: u16,
    pub lifetime: u16,
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
            _ => {}
        }
    }
}

pub const SLASH: Projectile = Projectile {
    pos: Vec2::ZERO,
    direction: Vec2::ZERO,
    speed: 5.0,
    drag: 0.15,
    draw_type: DrawType::Sprite(0.0, 0.0),
    chain: Vec::new(),
    life: 0,
    lifetime: 20,
};
