use crate::assets::Assets;
use macroquad::prelude::*;

pub enum ArmorType {
    Helmet,
    Chestplate,
}

pub struct Armor {
    ty: ArmorType,
    name: &'static str,
    sprite_x: f32,
    sprite_y: f32,
    stats: Stats,
}
pub fn get_movement_vector() -> Vec2 {
    let mut move_vector = Vec2::new(0.0, 0.0);
    if is_key_down(KeyCode::A) {
        move_vector.x -= 1.0
    }
    if is_key_down(KeyCode::D) {
        move_vector.x += 1.0
    }
    if is_key_down(KeyCode::W) {
        move_vector.y -= 1.0
    }
    if is_key_down(KeyCode::S) {
        move_vector.y += 1.0
    }
    if move_vector.x != 0.0 && move_vector.y != 0.0 {
        move_vector = move_vector.normalize();
    }
    move_vector
}

#[derive(Default)]
pub struct Stats {
    pub speed: f32,
    pub lives: u16,
}

#[derive(Default)]
pub struct Player {
    pub pos: Vec2,
    pub stats: Stats,
    pub helmet: Option<Armor>,
    pub chestplate: Option<Armor>,
    pub facing_left: bool,
    pub moving: bool,
    pub anim_frame: u32,
}
impl Player {
    pub fn draw(&self, assets: &Assets) {
        let x = self.pos.x.floor();
        let y = self.pos.y.floor();
        let draw_params = DrawTextureParams {
            flip_x: self.facing_left,
            ..Default::default()
        };
        let anim = if self.moving {
            (self.anim_frame as f32 * self.stats.speed / 5.0).floor() % 2.0
        } else {
            0.0
        };
        assets
            .entities
            .draw_sprite(x, y, anim, 0.0, Some(&draw_params));
        if let Some(chestplate) = &self.chestplate {
            assets.entities.draw_sprite(
                x,
                y,
                chestplate.sprite_x,
                chestplate.sprite_y,
                Some(&draw_params),
            );
        }
        if let Some(helmet) = &self.helmet {
            assets
                .entities
                .draw_sprite(x, y, helmet.sprite_x, helmet.sprite_y, Some(&draw_params));
        }
    }
}
