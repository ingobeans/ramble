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
use player::*;
use projectiles::*;

struct Ramble<'a> {
    assets: &'a Assets,
    player: Player,
    enemies: Vec<Enemy>,
    projectiles: Vec<Projectile>,
}
impl<'a> Ramble<'a> {
    fn new(assets: &'a Assets) -> Self {
        let mut player = Player::default();

        player.pos.x = SCREEN_WIDTH / 2.0;
        player.pos.y = SCREEN_HEIGHT / 2.0;
        player.stats.speed = 1.5;
        player.stats.roll_delay = 60.0;
        player.stats.max_lives = 3;
        player.lives = 3;
        player.hand = Some(assets.all_items[3].clone());

        Ramble {
            assets,
            player,
            enemies: Vec::new(),
            projectiles: Vec::new(),
        }
    }
    async fn run(&mut self) {
        let render_target = render_target(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);
        render_target.texture.set_filter(FilterMode::Nearest);
        let pixel_camera = Camera2D {
            render_target: Some(render_target),
            zoom: Vec2::new(1.0 / SCREEN_WIDTH * 2.0, 1.0 / SCREEN_HEIGHT * 2.0),
            target: Vec2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
            ..Default::default()
        };
        let mut last = get_time();
        self.enemies
            .push(Enemy::new(&ENEMY_TYPES[0], Vec2::new(10.0, 10.0)));
        self.enemies
            .push(Enemy::new(&ENEMY_TYPES[1], Vec2::new(40.0, 10.0)));
        self.enemies
            .push(Enemy::new(&ENEMY_TYPES[2], Vec2::new(5.0, 20.0)));

        loop {
            let (screen_width, screen_height) = screen_size();
            let scale_factor = (screen_width / SCREEN_WIDTH).min(screen_height / SCREEN_HEIGHT);
            let (mouse_x, mouse_y) = mouse_position();
            let (mouse_x, mouse_y) = (mouse_x / scale_factor, mouse_y / scale_factor);

            set_camera(&pixel_camera);
            clear_background(Color::from_hex(0x353658));

            let now = get_time();
            if now - last >= 1.0 / 60.0 {
                last = now;
                // update
                let (move_vector, speed) = if self.player.roll.0 == 0 {
                    (get_movement_vector(), self.player.stats().speed)
                } else {
                    (self.player.roll.1, 4.0)
                };
                self.player.pos += move_vector * speed;

                if move_vector != Vec2::ZERO {
                    self.player.moving = true;
                    self.player.anim_frame += self.player.stats.speed;
                } else {
                    self.player.moving = false;
                }

                // tick counters
                if self.player.attack_counter > 0.0 {
                    self.player.attack_counter -= 1.0
                }
                if self.player.roll_counter > 0.0 {
                    self.player.roll_counter -= 1.0
                }
                if self.player.roll.0 > 0 {
                    self.player.roll.0 -= 1;
                }
                if self.player.invuln_frames > 0 {
                    self.player.invuln_frames -= 1;
                }

                // player combat roll
                if is_key_down(KeyCode::Space)
                    && self.player.roll_counter <= 0.0
                    && self.player.moving
                {
                    self.player.roll_counter = self.player.stats().roll_delay;
                    self.player.roll = (12, move_vector)
                }

                // player attack
                if is_mouse_button_down(MouseButton::Left)
                    && self.player.attack_counter <= 0.0
                    && let Some(held) = &self.player.hand
                    && let ItemType::Held(held) = &held.ty
                {
                    let mut projectile = held.projectile.clone();
                    let delta = (Vec2::new(mouse_x, mouse_y) - self.player.pos).normalize();
                    projectile.pos = self.player.pos + delta * 10.0;
                    projectile.direction = delta;
                    projectile.player_owned = true;
                    projectile.stats = Some(held.stats.clone());
                    // make projectile travel faster if self.player is moving in same direction they're shooting
                    projectile.speed += 4.0 * (projectile.direction.dot(move_vector).max(0.0));
                    self.projectiles.push(projectile);
                    self.player.attack_counter = self.player.stats().attack_delay;
                }

                self.projectiles.retain_mut(|projectile| {
                    projectile.pos += projectile.direction * projectile.speed;
                    projectile.speed = projectile.speed.lerp(0.0, projectile.drag);
                    projectile.life += 1;
                    // check for collisions
                    if projectile.player_owned
                        && let Some(stats) = &projectile.stats
                    {
                        for enemy in self.enemies.iter_mut() {
                            if (enemy.pos - projectile.pos).length() < 4.0 {
                                for (_, amt) in &stats.damage {
                                    enemy.health -= amt;
                                }
                            }
                        }
                    } else if self.player.can_take_damage() {
                        let distance = (self.player.pos - projectile.pos).length();
                        if distance <= 4.0 {
                            self.player.damage();
                        }
                    }

                    projectile.life < projectile.lifetime
                });

                self.enemies.retain_mut(|enemy| {
                    let player_delta = (self.player.pos - enemy.pos);
                    match enemy.ty.movement {
                        EnemyMovement::Chase => {
                            let direction = player_delta.normalize();
                            enemy.pos += direction * enemy.ty.speed;
                            enemy.facing_left = direction.x < 0.0;
                            enemy.anim_frame += enemy.ty.speed;
                        }
                        EnemyMovement::Wander(face_player) => {
                            let mut new_target = true;
                            if let Some(move_target) = enemy.move_target {
                                new_target = false;
                                let delta = move_target - enemy.pos;
                                let direction = delta.normalize();
                                enemy.pos += direction * enemy.ty.speed;
                                if face_player {
                                    let player_dir = player_delta.normalize();
                                    enemy.facing_left = player_dir.x < 0.0;
                                } else {
                                    enemy.facing_left = direction.x < 0.0;
                                }
                                enemy.anim_frame += enemy.ty.speed;
                                if delta.length() <= 4.0 {
                                    new_target = true;
                                }
                            }
                            if new_target {
                                // set new move target if either no previous move target was set,
                                // or distance was less than 4.0
                                enemy.move_target = Some(Vec2::new(
                                    rand::gen_range(0.0, SCREEN_WIDTH),
                                    rand::gen_range(0.0, SCREEN_HEIGHT),
                                ));
                            }
                        }
                        _ => {}
                    }
                    // dmg player
                    if player_delta.length() <= 4.0 && self.player.can_take_damage() {
                        self.player.damage();
                    }

                    enemy.health > 0.0
                });
            }

            // draws
            for enemy in self.enemies.iter() {
                enemy.draw(self.assets);
            }

            self.player.draw(self.assets, mouse_x, mouse_y);

            for projectile in self.projectiles.iter() {
                projectile.draw(self.assets);
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
    let mut ramble = Ramble::new(&assets);
    ramble.run().await;
}
