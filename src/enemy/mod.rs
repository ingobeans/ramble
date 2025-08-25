use macroquad::prelude::*;

mod types;
pub use types::*;

use crate::{assets::Assets, consts::*, projectiles::Projectile};

pub enum EnemyMovement {
    /// Enemy chases player
    Chase,
    /// Enemy wanders randomly. Bool is whether it should face the player
    Wander(bool),
    /// Stands still
    Still,
}

pub enum ProjectileFiring {
    None,
    TowardsPlayer(Projectile, u8),
}

pub struct EnemyType {
    pub sprite_x: f32,
    pub sprite_y: f32,
    pub speed: f32,
    pub movement: EnemyMovement,
    pub projectile_firing: ProjectileFiring,
    pub frames: usize,
    pub max_health: f32,
}

pub struct Enemy {
    pub ty: &'static EnemyType,
    pub id: usize,
    pub pos: Vec2,
    pub facing_left: bool,
    pub anim_frame: f32,
    /// Used only for [EnemyMovement::Wander]
    pub move_target: Option<Vec2>,
    pub health: f32,
    pub damage_frames: u8,
    pub attack_counter: u8,
}
impl Enemy {
    pub fn new(ty: &'static EnemyType, pos: Vec2, id: usize) -> Self {
        Self {
            ty,
            id,
            pos,
            facing_left: false,
            anim_frame: 0.0,
            move_target: None,
            health: ty.max_health,
            damage_frames: 0,
            attack_counter: 0,
        }
    }
    pub fn draw(&self, assets: &Assets) {
        let x = self.pos.x.floor();
        let y = self.pos.y.floor();
        let draw_params = DrawTextureParams {
            flip_x: self.facing_left,
            ..Default::default()
        };
        if self.damage_frames > 0 {
            gl_use_material(&WHITE_MATERIAL);
        }
        let anim = (self.anim_frame / 3.0).floor() % self.ty.frames as f32;
        assets.entities.draw_sprite(
            x,
            y,
            self.ty.sprite_x + anim,
            self.ty.sprite_y,
            Some(&draw_params),
        );
        gl_use_default_material();
        // draw health bar
        let width = 12.0;
        let start_x = x - 8.0 + (16.0 - width) / 2.0;
        let start_y = y - 8.0;
        draw_rectangle(start_x, start_y, width, 3.0, Color::from_hex(0x180d2f));
        draw_rectangle(
            start_x,
            start_y,
            self.health / self.ty.max_health * width,
            3.0,
            Color::from_hex(0x47f641),
        );
    }
}
