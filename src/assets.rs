use asefile::AsepriteFile;
use image::EncodableLayout;
use macroquad::prelude::*;

pub struct Assets {
    pub entities: Texture2D,
}

impl Default for Assets {
    fn default() -> Self {
        Self {
            entities: load_ase_texture(include_bytes!("../assets/entities.ase")),
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
