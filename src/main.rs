use std::f32::consts::PI;

use macroquad::{miniquad::window::screen_size, prelude::*};

mod assets;
mod dungeon;
mod enemy;
mod items;
mod particles;
mod player;
mod projectiles;
mod utils;
use assets::*;
use dungeon::*;
use enemy::*;
use items::*;
use player::*;
use projectiles::*;
use utils::*;

struct Ramble<'a> {
    assets: &'a Assets,
    player: Player,
    enemies: Vec<Enemy>,
    enemy_id: usize,
    projectiles: Vec<Projectile>,
    dungeon_manager: DungeonManager,
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
            enemy_id: 0,
            projectiles: Vec::new(),
            dungeon_manager: DungeonManager {
                world: &WORLD_FOREST,
                room_index: 0,
            },
        }
    }
    fn spawn_enemies(&mut self, buffer: &mut Vec<Enemy>) {
        for mut enemy in buffer.drain(..) {
            enemy.id = self.enemy_id;
            self.enemy_id += 1;
            self.enemies.push(enemy);
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
        self.spawn_enemies(&mut self.dungeon_manager.spawn_room());
        let mut paused = false;

        loop {
            let (screen_width, screen_height) = screen_size();
            let scale_factor = (screen_width / SCREEN_WIDTH).min(screen_height / SCREEN_HEIGHT);
            let (mouse_x, mouse_y) = mouse_position();
            let (mouse_x, mouse_y) = (mouse_x / scale_factor, mouse_y / scale_factor);

            set_camera(&pixel_camera);
            clear_background(Color::from_hex(0x353658));
            // draw background tiles
            for y in 0..TILES_HEIGHT {
                for x in 0..TILES_WIDTH {
                    self.assets.world.draw_sprite(
                        x as f32 * 16.0 + 8.0,
                        y as f32 * 16.0 + 8.0,
                        0.0,
                        0.0,
                        None,
                    );
                }
            }

            let now = get_time();

            if is_key_pressed(KeyCode::Escape) {
                paused = !paused;
            }

            if !paused && now - last >= 1.0 / 60.0 {
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
                self.player.roll.0 = self.player.roll.0.saturating_sub(1);
                self.player.invuln_frames = self.player.invuln_frames.saturating_sub(1);

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
                            if !projectile.hit_enemies.contains(&enemy.id)
                                && (enemy.pos - projectile.pos).length() <= 8.0
                            {
                                for (_, amt) in &stats.damage {
                                    enemy.health -= amt;
                                }
                                enemy.damage_frames = 5;
                                projectile.hit_enemies.push(enemy.id);
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
                    let player_delta = self.player.pos - enemy.pos;
                    enemy.damage_frames = enemy.damage_frames.saturating_sub(1);
                    // move
                    match enemy.ty.movement {
                        EnemyMovement::Chase => {
                            enemy.direction = player_delta.normalize();
                            enemy.pos += enemy.direction * enemy.ty.speed;
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
                                    enemy.direction = player_delta.normalize();
                                } else {
                                    enemy.direction = direction;
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
                        EnemyMovement::Still => {
                            enemy.direction = RIGHT;
                        }
                    }
                    // shoot
                    if enemy.attack_counter == 0 {
                        match &enemy.ty.projectile_firing {
                            ProjectileFiring::Forwards(projectile, delay) => {
                                enemy.attack_counter = *delay;
                                let mut projectile = projectile.clone();
                                projectile.pos = enemy.pos;
                                projectile.direction = enemy.direction;
                                projectile.player_owned = false;
                                self.projectiles.push(projectile);
                            }
                            ProjectileFiring::Cardinally(projectile, delay) => {
                                enemy.attack_counter = *delay;
                                for i in 0..4 {
                                    let angle = enemy.direction.to_angle() + i as f32 * PI / 2.0;
                                    let direction = Vec2::from_angle(angle);
                                    let mut projectile = projectile.clone();
                                    projectile.pos = enemy.pos;
                                    projectile.direction = direction;
                                    projectile.player_owned = false;
                                    self.projectiles.push(projectile);
                                }
                            }
                            ProjectileFiring::None => {}
                        }
                    } else {
                        enemy.attack_counter -= 1
                    }

                    // dmg player on contact
                    if player_delta.length() <= 4.0 && self.player.can_take_damage() {
                        self.player.damage();
                    }

                    enemy.health > 0.0
                });
            }

            // check whether round complete
            if self.enemies.is_empty() {
                self.dungeon_manager.room_index += 1;
                self.spawn_enemies(&mut self.dungeon_manager.spawn_room());
            }

            // draws
            for enemy in self.enemies.iter() {
                enemy.draw(self.assets);
            }

            self.player.draw(self.assets, mouse_x, mouse_y);

            for projectile in self.projectiles.iter() {
                projectile.draw(self.assets);
            }

            // draw ui
            let max = self.player.stats().max_lives;
            for i in 0..max {
                let sprite = if i < self.player.lives { 0.0 } else { 1.0 };
                self.assets.ui.draw_sprite(
                    SCREEN_WIDTH / 2.0 - 8.0 * max as f32 + i as f32 * 16.0 + 8.0,
                    SCREEN_HEIGHT - 8.0,
                    sprite,
                    0.0,
                    None,
                );
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
