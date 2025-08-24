use macroquad::prelude::*;

mod assets;
use assets::*;

struct Ramble<'a> {
    assets: &'a Assets,
}

#[macroquad::main("ramble")]
async fn main() {
    let assets = Assets::default();
    loop {
        draw_texture(&assets.entities, 0.0, 0.0, WHITE);
        next_frame().await
    }
}
