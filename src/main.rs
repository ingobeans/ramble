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
fn window_conf() -> Conf {
    Conf {
        window_title: "ramble".to_string(),
        window_width: SCREEN_WIDTH as i32 * 3,
        window_height: SCREEN_HEIGHT as i32 * 3,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let assets = Assets::default();
    let mut player = Player::default();
    player.pos.x = SCREEN_WIDTH / 2.0;
    player.pos.y = SCREEN_HEIGHT / 2.0;
    player.stats.speed = 1.0;

    let render_target = render_target(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);
    render_target.texture.set_filter(FilterMode::Nearest);
    let pixel_camera = Camera2D {
        render_target: Some(render_target),
        zoom: Vec2::new(1.0 / SCREEN_WIDTH * 2.0, 1.0 / SCREEN_HEIGHT * 2.0),
        target: Vec2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
        ..Default::default()
    };
    let mut last = get_time();

    loop {
        let (screen_width, screen_height) = screen_size();
        let scale_factor = (screen_width / SCREEN_WIDTH).min(screen_height / SCREEN_HEIGHT);
        set_camera(&pixel_camera);
        clear_background(WHITE);

        let now = get_time();
        if now - last >= 1.0 / 60.0 {
            // update
            let move_vector = get_movement_vector();
            player.pos += move_vector * player.stats.speed;
            if move_vector != Vec2::ZERO {
                player.moving = true;
                player.anim_frame += 1;
            } else {
                player.moving = false;
            }
            if move_vector.x < 0.0 {
                player.facing_left = true;
            } else if move_vector.x > 0.0 {
                player.facing_left = false;
            }
            last = now;
        }

        // draws
        player.draw(&assets);

        // draw pixel camera to actual screen
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
