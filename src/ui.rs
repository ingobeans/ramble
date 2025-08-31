use macroquad::prelude::*;

use crate::{
    assets::Assets,
    items::{Item, ItemType},
    player::{ChaosCurse, Player},
    utils::*,
};

#[derive(Default)]
pub struct UiManager {
    pub inv_open: bool,
    pub cursor_item: Option<Item>,
}

impl UiManager {
    #[must_use]
    /// Update and draw UI. Returns if any item should be dropped.
    pub fn update(
        &mut self,
        assets: &Assets,
        player: &mut Player,
        mouse_x: f32,
        mouse_y: f32,
        ui_width: f32,
    ) -> Option<Item> {
        let max = player.stats().max_lives;
        for i in 0..max {
            let sprite = if i < player.stats().lives { 0.0 } else { 1.0 };
            assets.ui.draw_sprite(
                SCREEN_WIDTH / 2.0 - 8.0 * max as f32 + i as f32 * 16.0 + 8.0,
                SCREEN_HEIGHT - 8.0,
                sprite,
                0.0,
                None,
            );
        }
        if is_key_pressed(KeyCode::Escape) {
            self.inv_open = !self.inv_open;
        }

        if self.inv_open {
            let width = INV_SLOTS as f32 * 14.0 + 2.0;
            let height = 64.0;
            let x = (SCREEN_WIDTH - width) / 2.0;
            let y = (SCREEN_HEIGHT - height) / 2.0;
            draw_ui_rect(x, y, width, height);
            draw_ui_rect(x + 2.0, y + 2.0, 25.0, 25.0);

            let sx = x + 2.0 + 12.0;
            let sy = y + 2.0 + 12.0;
            player.draw_character(sx, sy, assets, 0.0, None);
            if let Some(held) = &player.hand {
                assets
                    .items
                    .draw_sprite(sx + 3.0, sy + 2.0, held.sprite_x, held.sprite_y, None);
            }
            let mut hovered = None;

            let sx = x + 2.0 + 25.0 + 2.0;
            let mut sy = y + 2.0;
            // helmet
            let helmet_is_none = player.helmet.is_none();
            if draw_slot(player.helmet.as_ref(), sx, sy, mouse_x, mouse_y, assets)
                && self
                    .cursor_item
                    .as_ref()
                    .is_none_or(|f| matches!(f.ty, ItemType::Helmet))
            {
                hovered = Some(&mut player.helmet);
            }
            if helmet_is_none {
                assets.items.draw_sprite(sx + 6.0, sy + 6.0, 0.0, 1.0, None);
            }

            // chestplate
            let chestplate_is_none = player.chestplate.is_none();
            sy += 12.0 + 1.0;
            if draw_slot(player.chestplate.as_ref(), sx, sy, mouse_x, mouse_y, assets)
                && self
                    .cursor_item
                    .as_ref()
                    .is_none_or(|f| matches!(f.ty, ItemType::Chestplate))
            {
                hovered = Some(&mut player.chestplate)
            }
            if chestplate_is_none {
                assets.items.draw_sprite(sx + 6.0, sy + 6.0, 0.0, 0.0, None);
            }

            // hand
            let hand_is_none = player.hand.is_none();
            let sx = x + 2.0;
            let sy = y + 25.0 + 3.0;
            if draw_slot(player.hand.as_ref(), sx, sy, mouse_x, mouse_y, assets)
                && self
                    .cursor_item
                    .as_ref()
                    .is_none_or(|f| matches!(f.ty, ItemType::Held(_)))
            {
                hovered = Some(&mut player.hand)
            }
            if hand_is_none {
                assets.items.draw_sprite(sx + 6.0, sy + 6.0, 0.0, 2.0, None);
            }

            // talismans
            for (index, slot) in player.talismans.iter_mut().enumerate() {
                let sx = x + width - 2.0 - 12.0;
                let sy = y + 2.0 + (12.0 + 1.0) * index as f32;
                draw_ui_rect(sx, sy, 12.0, 12.0);
                let is_none = slot.is_none();
                if draw_slot(slot.as_ref(), sx, sy, mouse_x, mouse_y, assets)
                    && self
                        .cursor_item
                        .as_ref()
                        .is_none_or(|f| matches!(f.ty, ItemType::Talisman))
                {
                    hovered = Some(slot)
                }
                if is_none {
                    assets.items.draw_sprite(sx + 6.0, sy + 6.0, 0.0, 3.0, None);
                }
            }
            let inventory_curse_count = player
                .curses
                .iter()
                .filter(|f| matches!(*f, ChaosCurse::LessInventory))
                .count();

            // inventory
            for (index, slot) in player.inventory.iter_mut().enumerate() {
                let sx = x + 2.0 + (12.0 + 2.0) * index as f32;
                let sy = y + height - 2.0 - 12.0;
                draw_ui_rect(sx, sy, 12.0, 12.0);
                if index < inventory_curse_count * 2 {
                    draw_line(sx + 12.0, sy, sx, sy + 12.0, 1.0, Color::from_hex(0xda2424));
                } else if draw_slot(slot.as_ref(), sx, sy, mouse_x, mouse_y, assets) {
                    hovered = Some(slot)
                }
            }
            if self.cursor_item.is_some() {
                draw_slot(
                    self.cursor_item.as_ref(),
                    mouse_x - 6.0,
                    mouse_y - 6.0,
                    0.0,
                    0.0,
                    assets,
                );
            }
            if self.cursor_item.is_none()
                && let Some(Some(item)) = hovered
            {
                draw_hover_item(&item, mouse_x, mouse_y, assets);
            }

            if is_mouse_button_pressed(MouseButton::Left) {
                if let Some(hovered) = hovered {
                    // if a slot is hovered,  replace it with the cursor item

                    std::mem::swap(&mut self.cursor_item, hovered);
                } else if (!(x..x + width).contains(&mouse_x)
                    || !(y..y + height).contains(&mouse_y))
                    && let Some(cursor_item) = self.cursor_item.take()
                {
                    // if cursor is outside inventory, and the cursor item isnt None, drop it.
                    return Some(cursor_item);
                }
            }
        }

        let width = 19.0 * 4.0 + 8.0 + 4.0;
        let height = 12.0;
        for (index, curse) in player.curses.iter().rev().enumerate() {
            let x = ui_width - (ui_width - SCREEN_WIDTH) / 2.0 - width;
            let y = index as f32 * (height - 1.0);
            draw_ui_rect(x, y, width, height);
            assets
                .curses
                .draw_sprite(x + 4.0 + 2.0, y + 4.0 + 2.0, *curse as u8 as f32, 0.0, None);
            assets.draw_text(
                &replace_pascal_case(&format!("{curse:?}")),
                x + 4.0 + 8.0,
                y + 4.0,
            );
        }

        None
    }
}

pub fn draw_hover_item(item: &Item, x: f32, y: f32, assets: &Assets) {
    let width = 128.0;
    let height = 32.0;
    draw_ui_rect(x, y, width, height);
    draw_ui_rect(x + 2.0, y + 2.0, 12.0, 12.0);
    item.ty.draw_icon(x + 2.0 + 6.0, y + 2.0 + 6.0, assets);
    let x = x + 4.0 + 12.0;
    let mut y = y + 2.0;
    assets.draw_text(&item.name(), x, y);
    y += 5.0;
    for line in item.stats().to_text() {
        assets.draw_text(&line, x, y);
        y += 5.0;
    }
}

pub fn draw_item_tooltip(item: &Item, assets: &Assets) {
    draw_hover_item(
        item,
        (SCREEN_WIDTH - 128.0) / 2.0,
        SCREEN_HEIGHT - 63.0,
        assets,
    );
}

pub fn draw_tooltip(text: &str, assets: &Assets) {
    let width = text.chars().count() as f32 * 4.0 + 4.0;
    let y = SCREEN_HEIGHT - 32.0;
    let x = (SCREEN_WIDTH - width) / 2.0;
    draw_ui_rect(x, y, width, 8.0);
    assets.draw_text(text, x + 2.0, y + 2.0);
}

pub fn draw_slot(
    item: Option<&Item>,
    x: f32,
    y: f32,
    mouse_x: f32,
    mouse_y: f32,
    assets: &Assets,
) -> bool {
    draw_ui_rect(x, y, 12.0, 12.0);
    let hovered = (x..x + 12.0).contains(&mouse_x) && (y - 1.0..y + 12.0 + 1.0).contains(&mouse_y);
    if let Some(item) = item {
        if let Some(e) = &item.enchantment {
            assets
                .enchantments
                .draw_sprite(x + 6.0, y + 6.0, e.sprite_x, e.sprite_y, None);
        }
        assets
            .items
            .draw_sprite(x + 6.0, y + 6.0, item.sprite_x, item.sprite_y, None);
    }
    hovered
}

pub fn draw_button(
    text: &str,
    assets: &Assets,
    x: f32,
    y: f32,
    width: f32,
    mouse_x: f32,
    mouse_y: f32,
) -> bool {
    let height = 8.0;
    let hovered = (x..x + width).contains(&mouse_x) && (y..y + height).contains(&mouse_y);
    let offset_x = ((width - text.chars().count() as f32 * 5.0) / 2.0).floor();

    draw_ui_rect(x, y, width, height);
    assets.draw_text(text, x + 2.0 + offset_x, y + 2.0);
    if hovered {
        draw_rectangle_lines(x, y, width, height, 2.0, Color::from_hex(0x8a4926));
    }
    hovered && is_mouse_button_pressed(MouseButton::Left)
}

pub fn draw_ui_rect(x: f32, y: f32, w: f32, h: f32) {
    draw_rectangle(x, y, w, h, Color::from_hex(0x1e090d));
    draw_rectangle(
        x + 1.0,
        y + 1.0,
        w - 2.0,
        h - 2.0,
        Color::from_hex(0x461c14),
    );
}
