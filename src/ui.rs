use macroquad::prelude::*;

use crate::{assets::Assets, player::Player, utils::*};

#[derive(Default)]
pub struct UiManager {
    pub inv_open: bool,
}

impl UiManager {
    pub fn update(&mut self, assets: &Assets, player: &mut Player) {
        if is_key_pressed(KeyCode::Escape) {
            self.inv_open = !self.inv_open;
        }

        if self.inv_open {
            let width = 140.0;
            let height = 64.0;
            let x = (SCREEN_WIDTH - width) / 2.0;
            let y = (SCREEN_HEIGHT - height) / 2.0;
            draw_ui_rect(x, y, width, height);
            draw_ui_rect(x + 2.0, y + 2.0, 25.0, 25.0);
            player.draw_character(x + 2.0 + 12.0, y + 2.0 + 12.0, assets, 0.0, None);
            // helmet
            draw_ui_rect(x + 2.0 + 25.0 + 2.0, y + 2.0, 12.0, 12.0);
            if let Some(item) = &player.helmet {
                assets.items.draw_sprite(
                    x + 2.0 + 25.0 + 2.0 + 6.0,
                    y + 2.0 + 6.0,
                    item.sprite_x,
                    item.sprite_y,
                    None,
                );
            }
            // chestplate
            draw_ui_rect(x + 2.0 + 25.0 + 2.0, y + 2.0 + 12.0 + 1.0, 12.0, 12.0);
            if let Some(item) = &player.chestplate {
                assets.items.draw_sprite(
                    x + 2.0 + 25.0 + 2.0 + 6.0,
                    y + 2.0 + 12.0 + 6.0 + 1.0,
                    item.sprite_x,
                    item.sprite_y,
                    None,
                );
            }
            // inventory
            for (index, slot) in player.inventory.iter().enumerate() {
                let slot_x = x + 2.0 + (12.0 + 2.0) * index as f32;
                let slot_y = y + height - 2.0 - 12.0;
                draw_ui_rect(slot_x, slot_y, 12.0, 12.0);
                if let Some(item) = slot {
                    assets
                        .items
                        .draw_sprite(slot_x, slot_y, item.sprite_x, item.sprite_y, None);
                }
            }
        }
    }
}

fn draw_ui_rect(x: f32, y: f32, w: f32, h: f32) {
    draw_rectangle(x, y, w, h, Color::from_hex(0x1e090d));
    draw_rectangle(
        x + 1.0,
        y + 1.0,
        w - 2.0,
        h - 2.0,
        Color::from_hex(0x461c14),
    );
}
