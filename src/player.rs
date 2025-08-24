use crate::{assets::Assets, items::Item};
use macroquad::prelude::*;

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
    pub max_lives: u16,
}

#[derive(Default)]
pub struct Player {
    pub pos: Vec2,
    pub lives: u16,
    pub stats: Stats,
    pub helmet: Option<Item>,
    pub chestplate: Option<Item>,
    pub hand: Option<Item>,
    pub offhand: Option<Item>,
    pub moving: bool,
    pub anim_frame: f32,
}
impl Player {
    pub fn draw(&self, assets: &Assets, mouse_x: f32, mouse_y: f32) {
        let x = self.pos.x.floor();
        let y = self.pos.y.floor();

        let facing_left = mouse_x < x;

        let draw_params = DrawTextureParams {
            flip_x: facing_left,
            ..Default::default()
        };
        let anim = if self.moving {
            (self.anim_frame / 5.0).floor() % 2.0
        } else {
            0.0
        };

        assets
            .entities
            .draw_sprite(x, y, anim, 0.0, Some(&draw_params));

        // draw armor
        if let Some(chestplate) = &self.chestplate {
            assets.items.draw_sprite(
                x,
                y,
                chestplate.sprite_x,
                chestplate.sprite_y,
                Some(&draw_params),
            );
        }
        if let Some(helmet) = &self.helmet {
            assets
                .items
                .draw_sprite(x, y, helmet.sprite_x, helmet.sprite_y, Some(&draw_params));
        }
        // draw held item
        if let Some(held) = &self.hand {
            let delta = Vec2::new(mouse_x, mouse_y) - self.pos;
            let angle = delta.to_angle();

            let draw_params = DrawTextureParams {
                rotation: angle,
                flip_y: facing_left,
                ..Default::default()
            };
            let offset = delta.normalize() * 12.0;
            assets.items.draw_sprite(
                x + offset.x,
                y + offset.y + 4.0,
                held.sprite_x,
                held.sprite_y,
                Some(&draw_params),
            );
        }
    }
}
