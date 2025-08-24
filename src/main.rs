use macroquad::{miniquad::window::screen_size, prelude::*};

mod assets;
mod consts;
mod player;
use assets::*;
use consts::*;
use player::*;

struct Ramble<'a> {
    assets: &'a Assets,
}

#[macroquad::main("ramble")]
async fn main() {
    let assets = Assets::default();
    let render_target = render_target(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);
    render_target.texture.set_filter(FilterMode::Nearest);
    let pixel_camera = Camera2D {
        render_target: Some(render_target),
        zoom: Vec2::new(1.0 / SCREEN_WIDTH * 2.0, 1.0 / SCREEN_HEIGHT * 2.0),
        target: Vec2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
        ..Default::default()
    };
    loop {
        let (screen_width, screen_height) = screen_size();
        let scale_factor = (screen_width / SCREEN_WIDTH).min(screen_height / SCREEN_HEIGHT);
        set_camera(&pixel_camera);
        assets.entities.draw_sprite(0.0, 0.0, 0.0, 1.0);
        set_default_camera();
        draw_texture_ex(
            &pixel_camera.render_target.as_ref().unwrap().texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(
                    SCREEN_WIDTH * scale_factor,
                    SCREEN_HEIGHT * scale_factor,
                )),
                ..Default::default()
            },
        );
        next_frame().await
    }
}
