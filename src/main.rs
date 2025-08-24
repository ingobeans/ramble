use macroquad::{miniquad::window::screen_size, prelude::*};

mod assets;
mod consts;
mod enemy;
mod items;
mod particles;
mod player;
mod projectiles;
use assets::*;
use consts::*;
use enemy::*;
use items::*;
use particles::*;
use player::*;
use projectiles::*;

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
    rand::srand(macroquad::miniquad::date::now() as _);

    let assets = Assets::default();
    let mut player = Player::default();

    player.pos.x = SCREEN_WIDTH / 2.0;
    player.pos.y = SCREEN_HEIGHT / 2.0;
    player.stats.speed = 1.5;
    player.chestplate = Some(ITEMS[0].clone());
    player.hand = Some(ITEMS[3].clone());

    let mut enemies: Vec<Enemy> = vec![Enemy {
        pos: Vec2::new(10.0, 10.0),
        ty: &ENEMY_TYPES[0],
        facing_left: false,
        anim_frame: 0.0,
        move_target: None,
    }];
    let mut projectiles: Vec<Projectile> = Vec::new();

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
        let (mouse_x, mouse_y) = mouse_position();
        let (mouse_x, mouse_y) = (mouse_x / scale_factor, mouse_y / scale_factor);

        set_camera(&pixel_camera);
        clear_background(Color::from_hex(0x353658));

        let now = get_time();
        if now - last >= 1.0 / 60.0 {
            // update
            let move_vector = get_movement_vector();
            player.pos += move_vector * player.stats().speed;
            if move_vector != Vec2::ZERO {
                player.moving = true;
                player.anim_frame += player.stats.speed;
            } else {
                player.moving = false;
            }
            last = now;
            if player.attack_counter > 0.0 {
                player.attack_counter -= 1.0
            }

            if is_mouse_button_down(MouseButton::Left)
                && player.attack_counter <= 0.0
                && let Some(held) = &player.hand
                && let ItemType::Held(held) = &held.ty
            {
                let mut projectile = held.projectile.clone();
                let delta = (Vec2::new(mouse_x, mouse_y) - player.pos).normalize();
                projectile.pos = player.pos + delta * 10.0;
                projectile.direction = delta;
                // make projectile travel faster if player is moving in same direction they're shooting
                projectile.speed *= 1.6_f32.powf(projectile.direction.dot(move_vector));
                projectiles.push(projectile);
                player.attack_counter = player.stats().attack_delay;
            }

            projectiles.retain_mut(|projectile| {
                projectile.pos += projectile.direction * projectile.speed;
                projectile.speed = projectile.speed.lerp(0.0, projectile.drag);
                projectile.life += 1;
                projectile.life < projectile.lifetime
            });

            for enemy in enemies.iter_mut() {
                match enemy.ty.movement {
                    EnemyMovement::Chase => {
                        let direction = (player.pos - enemy.pos).normalize();
                        enemy.pos += direction * enemy.ty.speed;
                        enemy.facing_left = direction.x < 0.0;
                        enemy.anim_frame += enemy.ty.speed;
                    }
                    EnemyMovement::Wander => {
                        if let Some(move_target) = enemy.move_target {
                            let delta = move_target - enemy.pos;
                            let direction = delta.normalize();
                            enemy.pos += direction * enemy.ty.speed;
                            enemy.facing_left = direction.x < 0.0;
                            enemy.anim_frame += enemy.ty.speed;
                            if delta.length() > 4.0 {
                                continue;
                            }
                        }
                        // set new move target if either no previous move target was set,
                        // or distance was less than 4.0
                        enemy.move_target = Some(Vec2::new(
                            rand::gen_range(0.0, SCREEN_WIDTH),
                            rand::gen_range(0.0, SCREEN_HEIGHT),
                        ));
                    }
                    _ => {}
                }
            }
        }

        // draws
        player.draw(&assets, mouse_x, mouse_y);

        for projectile in projectiles.iter() {
            projectile.draw(&assets);
        }

        for enemy in enemies.iter() {
            enemy.draw(&assets);
        }

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
