use std::collections::HashMap;

use crate::{
    assets::Assets,
    items::{Item, ItemType},
    projectiles::DamageType,
    utils::*,
};
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
    move_vector.try_normalize().unwrap_or(Vec2::ZERO)
}

#[derive(Default, Clone)]
pub struct Stats {
    pub speed: f32,
    pub max_lives: u16,
    pub attack_delay: f32,
    pub roll_delay: f32,
    pub damage: HashMap<DamageType, f32>,
}
impl Stats {
    pub fn merge(&mut self, other: &Stats) {
        self.speed += other.speed;
        self.max_lives += other.max_lives;
        self.attack_delay += other.attack_delay;
        self.roll_delay += other.roll_delay;
    }
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
    pub attack_counter: f32,
    pub invuln_frames: u8,
    pub roll_counter: f32,
    /// Info about current roll. First value is roll frames, if zero, player is not rolling.
    /// Second is roll direction.
    pub roll: (u8, Vec2),
}
impl Player {
    pub fn stats(&self) -> Stats {
        let mut stats = self.stats.clone();
        for item in [&self.helmet, &self.chestplate, &self.hand, &self.offhand] {
            if let Some(item) = item {
                stats.merge(&item.stats);
                if let ItemType::Held(held) = &item.ty {
                    stats.merge(&held.stats);
                }
            }
        }
        stats
    }
    pub fn can_take_damage(&self) -> bool {
        self.roll.0 == 0 && self.invuln_frames == 0
    }
    pub fn damage(&mut self) {
        self.invuln_frames = 100;
        self.lives -= 1;
        if self.lives == 0 {
            todo!("game over!");
        }
    }
    pub fn draw(&self, assets: &Assets, mouse_x: f32, mouse_y: f32) {
        let x = self.pos.x.floor();
        let y = self.pos.y.floor();

        let facing_left = mouse_x < x;

        let draw_params = DrawTextureParams {
            flip_x: facing_left,
            ..Default::default()
        };

        // make player flash white
        if self.invuln_frames != 0 && (self.invuln_frames as f32 / 10.0).floor() % 2.0 == 1.0 {
            gl_use_material(&WHITE_MATERIAL);
        }

        if self.roll.0 != 0 {
            let anim = (self.roll.0 / 3) as f32 % 4.0;

            assets
                .entities
                .draw_sprite(x, y, 2.0 + anim, 0.0, Some(&draw_params));
        } else {
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
                assets.items.draw_sprite(
                    x,
                    y,
                    helmet.sprite_x,
                    helmet.sprite_y,
                    Some(&draw_params),
                );
            }
        }

        gl_use_default_material();

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
                y + offset.y + 2.0,
                held.sprite_x,
                held.sprite_y,
                Some(&draw_params),
            );
        }
    }
}
