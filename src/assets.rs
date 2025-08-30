use asefile::AsepriteFile;
use hashmap_macro::hashmap;
use image::EncodableLayout;
use macroquad::prelude::*;

use crate::{
    items::{Enchantment, Item, get_enchantments, get_items},
    utils::*,
};

pub struct Assets {
    pub all_items: Vec<Item>,
    pub all_enchantments: Vec<Enchantment>,
    pub entities: Spritesheet,
    pub items: Spritesheet,
    pub particles: Spritesheet,
    pub ui: Spritesheet,
    pub world: Spritesheet,
    pub shop: Texture2D,
    pub curses: Spritesheet,
    pub enchantments: Spritesheet,
    font: Spritesheet,
}
impl Default for Assets {
    fn default() -> Self {
        Self {
            entities: Spritesheet::new(
                load_ase_texture(include_bytes!("../assets/entities.ase"), None),
                16.0,
            ),
            items: Spritesheet::new(
                load_ase_texture(include_bytes!("../assets/items.ase"), Some(1)),
                16.0,
            ),
            particles: Spritesheet::new(
                load_ase_texture(include_bytes!("../assets/particles.ase"), None),
                16.0,
            ),
            ui: Spritesheet::new(
                load_ase_texture(include_bytes!("../assets/ui.ase"), None),
                16.0,
            ),
            world: Spritesheet::new(
                load_ase_texture(include_bytes!("../assets/world.ase"), None),
                16.0,
            ),
            shop: load_ase_texture(include_bytes!("../assets/shop.ase"), None),
            curses: Spritesheet::new(
                load_ase_texture(include_bytes!("../assets/curses.ase"), None),
                8.0,
            ),
            enchantments: Spritesheet::new(
                load_ase_texture(include_bytes!("../assets/enchantments.ase"), None),
                16.0,
            ),
            font: Spritesheet::new(
                load_ase_texture(include_bytes!("../assets/font.ase"), None),
                4.0,
            ),
            all_items: get_items(),
            all_enchantments: get_enchantments(),
        }
    }
}
impl Assets {
    pub fn get_item_by_name(&self, name: &str) -> &Item {
        for item in self.all_items.iter() {
            if item.name == name {
                return item;
            }
        }
        panic!()
    }
    pub fn draw_text(&self, text: &str, mut x: f32, mut y: f32) -> (f32, f32) {
        let original_x = x;
        let original_y = y;
        let hardcoded =
            hashmap!(':'=>36,'.'=>37,'-'=>38,'%'=>39,'+'=>40,'/'=>41,'H'=>42,'('=>43,')'=>44);
        gl_use_material(&COLOR_MOD_MATERIAL);
        COLOR_MOD_MATERIAL.set_uniform("color", COLORS[1]);

        for char in text.chars() {
            if char == '\n' {
                y += 5.0;
                x = original_x;
                continue;
            } else if char == ' ' {
                x += 4.0;
                continue;
            }
            let code = char as u8;
            if code < COLORS.len() as u8 {
                COLOR_MOD_MATERIAL.set_uniform("color", COLORS[code as usize]);
            }

            let index = if let Some(value) = hardcoded.get(&char) {
                *value
            } else if code.is_ascii_lowercase() {
                code - b'a'
            } else if code.is_ascii_digit() {
                code - b'0' + 26
            } else {
                continue;
            };
            self.font
                .draw_sprite(x + 2.0, y + 2.0, index as f32, 0.0, None);

            x += 4.0
        }

        COLOR_MOD_MATERIAL.set_uniform("color", COLORS[0]);
        gl_use_default_material();
        (x - original_x, y - original_y)
    }
}

fn load_ase_texture(bytes: &[u8], layer: Option<u32>) -> Texture2D {
    let img = AsepriteFile::read(bytes).unwrap();
    let img = if let Some(layer) = layer {
        img.layer(layer).frame(0).image()
    } else {
        img.frame(0).image()
    };
    let new = Image {
        width: img.width() as u16,
        height: img.height() as u16,
        bytes: img.as_bytes().to_vec(),
    };
    let texture = Texture2D::from_image(&new);
    texture.set_filter(FilterMode::Nearest);
    texture
}

pub struct Spritesheet {
    pub texture: Texture2D,
    pub sprite_size: f32,
}
impl Spritesheet {
    pub fn new(texture: Texture2D, sprite_size: f32) -> Self {
        Self {
            texture,
            sprite_size,
        }
    }
    pub fn draw_sprite(
        &self,
        screen_x: f32,
        screen_y: f32,
        tile_x: f32,
        tile_y: f32,
        params: Option<&DrawTextureParams>,
    ) {
        let mut p = params.cloned().unwrap_or(DrawTextureParams::default());
        p.dest_size = p
            .dest_size
            .or(Some(Vec2::new(self.sprite_size, self.sprite_size)));
        p.source = p.source.or(Some(Rect {
            x: tile_x * self.sprite_size,
            y: tile_y * self.sprite_size,
            w: self.sprite_size,
            h: self.sprite_size,
        }));
        draw_texture_ex(
            &self.texture,
            screen_x - self.sprite_size / 2.0,
            screen_y - self.sprite_size / 2.0,
            WHITE,
            p,
        );
    }
}
