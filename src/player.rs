use std::collections::HashMap;

use crate::{
    assets::Assets,
    items::{Item, ItemType},
    projectiles::{DamageType, Projectile},
    utils::*,
};
use enum_iterator::Sequence;
use macroquad::prelude::*;
use struct_iterable::Iterable;

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

#[derive(Clone, Copy, Debug, Sequence)]
pub enum ChaosCurse {
    EnemyShields,
    AcidPuddles,
    LessInventory,
    BonusEnemies,
    RefillHealth,
    HalvedHolyDmg,
    DoubledUnholyDmg,
    Gift,
    RepairArmor,
}

#[derive(Default, Clone, Iterable)]
pub struct Stats {
    pub move_speed: f32,
    pub move_speed_mod: f32,
    pub attack_delay_mod: f32,
    pub attack_delay: f32,
    pub roll_delay: f32,
    pub roll_delay_mod: f32,
    pub max_lives: u16,
    pub lives: u16,
    pub damage: HashMap<DamageType, f32>,
    pub damage_modifiers: HashMap<DamageType, f32>,
    pub on_hit_effects: HashMap<Option<DamageType>, Vec<(Projectile, HashMap<DamageType, f32>)>>,
}
impl Stats {
    pub fn to_text(&self) -> Vec<String> {
        fn damage_modifiers_to_text(damage_modifiers: &HashMap<DamageType, f32>) -> Vec<String> {
            let mut lines = Vec::new();

            for (k, v) in damage_modifiers {
                if *v == f32::default() {
                    continue;
                }
                lines.push(format!(
                    "\x00{} damage\x01: {:+}%",
                    k.to_text(),
                    (v * 100.0).round()
                ));
            }
            lines
        }
        fn damage_to_text(damage: &HashMap<DamageType, f32>) -> Vec<String> {
            let mut lines = Vec::new();

            for (k, v) in damage {
                if *v == f32::default() {
                    continue;
                }
                lines.push(format!("\x00{} damage\x01: {v:.2}", k.to_text()));
            }
            lines
        }
        let mut lines = Vec::new();
        if self.max_lives != 0 {
            lines.push(format!(
                "\x02H\x01 lives: {}/{}",
                self.lives, self.max_lives
            ));
        }
        for (k, v) in self.iter() {
            if let Some(f) = v.downcast_ref::<f32>() {
                if *f == f32::default() {
                    continue;
                }
                let is_mod = k.ends_with("_mod");
                let mut formatted = k.replace("_", " ");
                if is_mod {
                    formatted = formatted.trim_end_matches(" mod").to_string();
                }
                if formatted.contains("move speed") || formatted.contains("roll delay") {
                    formatted = format!("\x04{formatted}\x01");
                }
                if formatted.contains("attack") {
                    formatted = format!("\x03{formatted}\x01");
                }
                if is_mod {
                    lines.push(format!("{}: {:+}%", formatted, (f * 100.0).round()));
                } else {
                    lines.push(format!("{}: {}", formatted, f.round()));
                }
            }
        }
        lines.append(&mut &mut damage_to_text(&self.damage));
        lines.append(&mut damage_modifiers_to_text(&self.damage_modifiers));
        for (k, v) in &self.on_hit_effects {
            if v.is_empty() {
                continue;
            }
            let text = k
                .map(|f| format!("{f:?} dmg dealt").to_lowercase())
                .unwrap_or("hit".into());
            let v = v
                .iter()
                .map(|f| damage_to_text(&f.1).join(&String::from("")))
                .collect::<Vec<_>>()
                .join(&String::from("\n& "));
            lines.push(format!("\x01on \x00{}\x01, deal:\n {}", text, v));
        }
        lines
    }
    pub fn merge(&mut self, other: &Stats) {
        self.max_lives += other.max_lives;
        self.lives += other.lives;
        self.move_speed_mod += other.move_speed_mod;
        self.attack_delay_mod += other.attack_delay_mod;
        self.attack_delay += other.attack_delay;
        self.roll_delay_mod += other.roll_delay_mod;
        for (k, v) in &other.damage_modifiers {
            if self.damage_modifiers.contains_key(k) {
                self.damage_modifiers
                    .insert(*k, self.damage_modifiers[k] + *v);
            } else {
                self.damage_modifiers.insert(*k, *v);
            }
        }
        for (k, v) in &other.damage {
            if self.damage.contains_key(k) {
                self.damage.insert(*k, self.damage[k] + *v);
            } else {
                self.damage.insert(*k, *v);
            }
        }
        for (k, v) in &other.on_hit_effects {
            if let Some(value) = self.on_hit_effects.get_mut(k) {
                value.append(&mut v.clone());
            } else {
                self.on_hit_effects.insert(*k, v.clone());
            }
        }
    }
    pub fn apply_modifiers(&mut self) {
        self.move_speed *= 1.0 + self.move_speed_mod;
        self.attack_delay *= 1.0 + self.attack_delay_mod;
        self.roll_delay *= 1.0 + self.roll_delay_mod;
    }
}

#[derive(Default)]
pub struct Player {
    pub pos: Vec2,
    pub internal_stats: Stats,
    pub curses: Vec<ChaosCurse>,
    pub inventory: Vec<Option<Item>>,
    pub talismans: Vec<Option<Item>>,
    pub helmet: Option<Item>,
    pub chestplate: Option<Item>,
    pub hand: Option<Item>,
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
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            inventory: vec![None; INV_SLOTS],
            talismans: vec![None; 3],
            internal_stats: Stats {
                max_lives: 3,
                lives: 3,
                move_speed: 1.5,
                roll_delay: 60.0,
                ..Default::default()
            },
            ..Default::default()
        }
    }
    pub fn regen(&mut self) {
        self.internal_stats.lives = self.internal_stats.max_lives;
    }
    pub fn repair_armor(&mut self) {
        for armor in [&mut self.chestplate, &mut self.helmet] {
            if let Some(armor) = armor {
                armor.internal_stats.lives = armor.internal_stats.max_lives;
            }
        }
    }
    /// Panics if inventory full
    pub fn give_item(&mut self, item: Item) {
        match &item.ty {
            ItemType::Chestplate => {
                if self.chestplate.is_none() {
                    self.chestplate = Some(item);
                    return;
                }
            }
            ItemType::Helmet => {
                if self.helmet.is_none() {
                    self.helmet = Some(item);
                    return;
                }
            }
            ItemType::Held(_) => {
                if self.hand.is_none() {
                    self.hand = Some(item);
                    return;
                }
            }
            ItemType::Talisman => {
                for t in self.talismans.iter_mut() {
                    if t.is_none() {
                        *t = Some(item);
                        return;
                    }
                }
            }
        };
        let inventory_curse_count = self
            .curses
            .iter()
            .filter(|f| matches!(*f, ChaosCurse::LessInventory))
            .count();
        for slot in self.inventory.iter_mut().skip(inventory_curse_count * 2) {
            if slot.is_none() {
                *slot = Some(item);
                return;
            }
        }
        panic!("Inventory full!");
    }
    pub fn inv_slot_free(&self, ty: &ItemType) -> bool {
        if match ty {
            ItemType::Chestplate => self.chestplate.is_none(),
            ItemType::Helmet => self.helmet.is_none(),
            ItemType::Held(_) => self.hand.is_none(),
            ItemType::Talisman => {
                let mut y = false;
                for t in self.talismans.iter() {
                    if t.is_none() {
                        y = true;
                        break;
                    }
                }
                y
            }
        } {
            return true;
        }

        let inventory_curse_count = self
            .curses
            .iter()
            .filter(|f| matches!(*f, ChaosCurse::LessInventory))
            .count();
        for slot in self.inventory.iter().skip(inventory_curse_count * 2) {
            if slot.is_none() {
                return true;
            }
        }
        false
    }
    pub fn stats(&self) -> Stats {
        let mut stats = self.internal_stats.clone();

        let mut items = vec![&self.helmet, &self.chestplate, &self.hand];
        let t = self.talismans.clone();
        items.append(&mut t.iter().collect());
        for item in items.into_iter().flatten() {
            stats.merge(&item.stats());
        }
        stats.apply_modifiers();
        stats
    }
    pub fn can_take_damage(&self) -> bool {
        self.roll.0 == 0 && self.invuln_frames == 0
    }
    pub fn damage(&mut self) -> bool {
        self.invuln_frames = 100;
        // find where to take heart
        let mut items: Vec<&mut Option<Item>> = self.talismans.iter_mut().collect();
        items.append(&mut vec![
            &mut self.helmet,
            &mut self.chestplate,
            &mut self.hand,
        ]);
        for item in items {
            if let Some(item) = item
                && item.internal_stats.lives > 0
            {
                item.internal_stats.lives -= 1;
                return false;
            }
        }

        self.internal_stats.lives -= 1;
        // return whether game over
        self.internal_stats.lives == 0
    }
    pub fn draw_character(
        &self,
        x: f32,
        y: f32,
        assets: &Assets,
        anim: f32,
        draw_params: Option<&DrawTextureParams>,
    ) {
        assets.entities.draw_sprite(x, y, anim, 0.0, draw_params);

        // draw armor
        if let Some(chestplate) = &self.chestplate {
            assets
                .items
                .draw_sprite(x, y, chestplate.sprite_x, chestplate.sprite_y, draw_params);
        }
        if let Some(helmet) = &self.helmet {
            assets
                .items
                .draw_sprite(x, y, helmet.sprite_x, helmet.sprite_y, draw_params);
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
            gl_use_material(&COLOR_MOD_MATERIAL);
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
            self.draw_character(self.pos.x, self.pos.y, assets, anim, Some(&draw_params));
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
