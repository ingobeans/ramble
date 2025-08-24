use macroquad::prelude::*;

mod types;
pub use types::*;

use crate::assets::Assets;

pub enum EnemyMovement {
    Chase,
    Wander,
    Still,
}

pub struct EnemyType {
    pub sprite_x: f32,
    pub sprite_y: f32,
    pub speed: f32,
    pub movement: EnemyMovement,
    pub frames: usize,
}

pub struct Enemy {
    pub ty: &'static EnemyType,
    pub pos: Vec2,
    pub facing_left: bool,
    pub anim_frame: f32,
    /// Used only for [EnemyMovement::Wander]
    pub move_target: Option<Vec2>,
}
impl Enemy {
    pub fn draw(&self, assets: &Assets) {
        let x = self.pos.x.floor();
        let y = self.pos.y.floor();
        let draw_params = DrawTextureParams {
            flip_x: self.facing_left,
            ..Default::default()
        };
        let anim = (self.anim_frame / 3.0).floor() % self.ty.frames as f32;
        assets.entities.draw_sprite(
            x,
            y,
            self.ty.sprite_x + anim,
            self.ty.sprite_y,
            Some(&draw_params),
        );
    }
}
