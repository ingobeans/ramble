use asefile::AsepriteFile;
use image::EncodableLayout;
use macroquad::prelude::*;

pub struct Assets {
    pub entities: Spritesheet,
}

impl Default for Assets {
    fn default() -> Self {
        Self {
            entities: Spritesheet::new(
                load_ase_texture(include_bytes!("../assets/entities.ase")),
                16.0,
            ),
        }
    }
}

fn load_ase_texture(bytes: &[u8]) -> Texture2D {
    let img = AsepriteFile::read(bytes).unwrap();
    let img = img.frame(0).image();
    let new = Image {
        width: img.width() as u16,
        height: img.height() as u16,
        bytes: img.as_bytes().to_vec(),
    };
    Texture2D::from_image(&new)
}

pub struct Spritesheet {
    texture: Texture2D,
    sprite_size: f32,
}
impl Spritesheet {
    pub fn new(texture: Texture2D, sprite_size: f32) -> Self {
        Self {
            texture,
            sprite_size,
        }
    }
    pub fn draw_sprite(&self, screen_x: f32, screen_y: f32, tile_x: f32, tile_y: f32) {
        draw_texture_ex(
            &self.texture,
            screen_x,
            screen_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.sprite_size, self.sprite_size)),
                source: Some(Rect {
                    x: tile_x * self.sprite_size,
                    y: tile_y * self.sprite_size,
                    w: self.sprite_size,
                    h: self.sprite_size,
                }),
                ..Default::default()
            },
        );
    }
}
