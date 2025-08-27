use std::f32::consts::PI;

use macroquad::{miniquad::window::screen_size, prelude::*};

mod assets;
mod dungeon;
mod enemy;
mod items;
mod particles;
mod player;
mod projectiles;
mod ui;
mod utils;
mod worlds;
use assets::*;
use dungeon::*;
use enemy::*;
use items::*;
use player::*;
use projectiles::*;
use ui::UiManager;
use utils::*;
use worlds::*;

enum GameState {
    InRound,
    PostRound(u32),
    PreRound(u32),
}
impl GameState {
    fn should_draw(&self) -> bool {
        match self {
            GameState::InRound | GameState::PostRound(_) => true,
            GameState::PreRound(frame) => *frame >= 40 / 2,
        }
    }
    fn door_frame(&self) -> f32 {
        match self {
            GameState::InRound => 0.0,
            GameState::PostRound(frame) => (*frame as f32 / 10.0 * 4.0).floor().min(3.0) * 2.0,
            GameState::PreRound(frame) => {
                if *frame >= PREROUND_TRANSITION_TIME / 2 {
                    0.0
                } else {
                    6.0
                }
            }
        }
    }
}
struct Ramble<'a> {
    assets: &'a Assets,
    player: Player,
    state: GameState,
    enemies: Vec<Enemy>,
    dropped_items: Vec<(Vec2, Item)>,
    enemy_id: usize,
    projectiles: Vec<Projectile>,
    dungeon_manager: DungeonManager,
    ui_manager: UiManager,
}
impl<'a> Ramble<'a> {
    fn new(assets: &'a Assets) -> Self {
        let mut player = Player::new(Vec2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT - 16.0));
        player.hand = Some(assets.all_items[3].clone());
        player.helmet = Some(assets.all_items[1].clone());
        player.chestplate = Some(assets.all_items[0].clone());

        Ramble {
            assets,
            player,
            state: GameState::PostRound(0),
            enemies: Vec::new(),
            dropped_items: Vec::new(),
            enemy_id: 0,
            projectiles: Vec::new(),
            dungeon_manager: DungeonManager {
                world: &WORLD_FOREST,
                room_index: 0,
            },
            ui_manager: UiManager::default(),
        }
    }
    fn spawn_enemies(&mut self, buffer: &mut Vec<Enemy>) {
        for mut enemy in buffer.drain(..) {
            enemy.id = self.enemy_id;
            enemy.pos.y += 32.0;
            self.enemy_id += 1;
            self.enemies.push(enemy);
        }
    }
    fn update(&mut self, mouse_x: f32, mouse_y: f32) {
        let (move_vector, speed) = if self.player.roll.0 == 0 {
            (get_movement_vector(), self.player.stats().speed)
        } else {
            (self.player.roll.1, 4.0)
        };
        let max_y = 28.0;
        self.player.pos = (self.player.pos + move_vector * speed).clamp(
            Vec2::new(4.0, max_y),
            Vec2::new(SCREEN_WIDTH - 4.0, SCREEN_HEIGHT - 8.0),
        );
        let door_start_x = TILES_WIDTH / 2 - 1;

        // go to next room
        if let GameState::PostRound(_) = self.state
            && self.player.pos.y == 28.0
            && (door_start_x as f32 * 16.0 + 4.0..door_start_x as f32 * 16.0 + 28.0)
                .contains(&self.player.pos.x)
            && is_key_down(KeyCode::W)
        {
            self.player.pos = Vec2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT - 16.0);
            self.enemies.clear();
            self.spawn_enemies(&mut self.dungeon_manager.spawn_room());
            self.dungeon_manager.room_index += 1;
            self.state = GameState::PreRound(0);
        }

        if move_vector != Vec2::ZERO {
            self.player.moving = true;
            self.player.anim_frame += speed;
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
        if is_key_down(KeyCode::Space) && self.player.roll_counter <= 0.0 && self.player.moving {
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
            projectile.stats = Some(self.player.stats());
            // make projectile travel faster if self.player is moving in same direction they're shooting
            projectile.speed += 4.0 * (projectile.direction.dot(move_vector).max(0.0));
            self.projectiles.push(projectile);
            let stats = self.player.stats();
            self.player.attack_counter = stats.attack_delay;
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
                        for (k, mut amt) in stats.damage.clone() {
                            if let Some(modifier) = stats.damage_modifiers.get(&k) {
                                amt *= 1.0 + modifier;
                            }
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
        let enemy_positions: Vec<Vec2> = self.enemies.iter().map(|f| f.pos).collect();

        self.enemies.retain_mut(|enemy| {
            let player_delta = self.player.pos - enemy.pos;
            enemy.damage_frames = enemy.damage_frames.saturating_sub(1);
            let mut move_direction = Vec2::ZERO;
            // move
            match enemy.ty.movement {
                EnemyMovement::Chase => {
                    enemy.direction = player_delta.normalize();
                    move_direction = enemy.direction;
                }
                EnemyMovement::Wander(face_player) => {
                    let mut new_target = true;
                    if let Some(move_target) = enemy.move_target {
                        new_target = false;
                        let delta = move_target - enemy.pos;
                        move_direction = delta.normalize();
                        if face_player {
                            enemy.direction = player_delta.normalize();
                        } else {
                            enemy.direction = move_direction;
                        }
                        if delta.length() <= 4.0 {
                            new_target = true;
                        }
                    }
                    if new_target {
                        // set new move target if either no previous move target was set,
                        // or distance was less than 4.0
                        enemy.move_target = Some(Vec2::new(
                            rand::gen_range(0.0, SCREEN_WIDTH),
                            rand::gen_range(32.0, SCREEN_HEIGHT),
                        ));
                    }
                }
                EnemyMovement::Still => {
                    enemy.direction = RIGHT;
                }
            }
            for pos in enemy_positions.iter() {
                if pos != &enemy.pos {
                    let delta = enemy.pos - *pos;
                    if delta.length() <= 7.0 {
                        move_direction = delta.normalize();
                    }
                }
            }
            enemy.pos += move_direction * enemy.ty.speed;
            enemy.anim_frame += enemy.ty.speed;
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

        if let GameState::InRound = self.state
            && self.enemies.is_empty()
        {
            self.state = GameState::PostRound(0)
        }
    }

    fn draw(&mut self, mouse_x: f32, mouse_y: f32) {
        // draws
        for enemy in self.enemies.iter() {
            enemy.draw(self.assets);
        }

        let mut item_under_player: Option<(usize, f32)> = None;

        for (index, (pos, item)) in self.dropped_items.iter().enumerate() {
            ui::draw_slot(Some(item), pos.x - 6.0, pos.y - 6.0, 0.0, 0.0, self.assets);

            let dist = pos.distance(self.player.pos);
            if dist <= 7.0 && item_under_player.is_none_or(|f| f.1 > dist) {
                item_under_player = Some((index, dist));
            }
        }
        if let Some(item_under_player) = item_under_player {
            ui::draw_tooltip("press e to pick up", self.assets);
            if is_key_pressed(KeyCode::E) && self.player.inv_slot_free() {
                let item = self.dropped_items.remove(item_under_player.0).1;
                for slot in self.player.inventory.iter_mut() {
                    if slot.is_none() {
                        *slot = Some(item);
                        break;
                    }
                }
            }
        }

        self.player.draw(self.assets, mouse_x, mouse_y);

        for projectile in self.projectiles.iter() {
            projectile.draw(self.assets);
        }

        // draw ui
        let max = self.player.stats().max_lives;
        for i in 0..max {
            let sprite = if i < self.player.stats().lives {
                0.0
            } else {
                1.0
            };
            self.assets.ui.draw_sprite(
                SCREEN_WIDTH / 2.0 - 8.0 * max as f32 + i as f32 * 16.0 + 8.0,
                SCREEN_HEIGHT - 8.0,
                sprite,
                0.0,
                None,
            );
        }
        if let Some(dropped) =
            self.ui_manager
                .update(self.assets, &mut self.player, mouse_x, mouse_y)
        {
            let mut pos = self.player.pos;
            pos += (Vec2::new(mouse_x, mouse_y) - self.player.pos).normalize() * 3.0;
            self.dropped_items.push((pos, dropped));
        }
    }
    async fn run(&mut self) {
        let render_target = render_target(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);
        render_target.texture.set_filter(FilterMode::Nearest);
        let mut pixel_camera = Camera2D {
            render_target: Some(render_target),
            zoom: Vec2::new(1.0 / SCREEN_WIDTH * 2.0, 1.0 / SCREEN_HEIGHT * 2.0),
            target: Vec2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
            ..Default::default()
        };
        let mut last = get_time();

        loop {
            let (screen_width, screen_height) = screen_size();
            let scale_factor = (screen_width / SCREEN_WIDTH).min(screen_height / SCREEN_HEIGHT);
            let horizontal_padding = (screen_width - SCREEN_WIDTH * scale_factor) / 2.0;
            let (mouse_x, mouse_y) = mouse_position();
            let (mouse_x, mouse_y) = (
                (mouse_x - horizontal_padding) / scale_factor,
                mouse_y / scale_factor,
            );

            clear_background(Color::from_hex(0x180d2f));
            set_camera(&pixel_camera);
            clear_background(Color::from_hex(0x180d2f));
            // draw background tiles
            let screen_tiles = SCREEN_HEIGHT as u32 / 16;
            let door_start_x = TILES_WIDTH / 2 - 1;
            for y in 0..screen_tiles {
                for x in 0..TILES_WIDTH {
                    let tile = if y == 0 {
                        7.0
                    } else if x == 0 {
                        if y == 1 { 5.0 } else { 3.0 }
                    } else if x == TILES_WIDTH - 1 {
                        if y == 1 { 6.0 } else { 4.0 }
                    } else if y == 1 {
                        if x == door_start_x {
                            8.0 + self.state.door_frame()
                        } else if x == door_start_x + 1 {
                            9.0 + self.state.door_frame()
                        } else {
                            2.0
                        }
                    } else {
                        0.0
                    };
                    self.assets.world.draw_sprite(
                        x as f32 * 16.0 + 8.0,
                        y as f32 * 16.0 + 8.0,
                        tile,
                        0.0,
                        None,
                    );
                }
            }

            let now = get_time();

            if !self.ui_manager.inv_open && now - last >= 1.0 / 60.0 {
                // update
                last = now;

                match &mut self.state {
                    GameState::InRound => {
                        self.update(mouse_x, mouse_y);
                    }
                    GameState::PostRound(frame) => {
                        *frame = frame.saturating_add(1);
                        self.update(mouse_x, mouse_y);
                    }
                    GameState::PreRound(frame) => {
                        let max = PREROUND_TRANSITION_TIME + PREROUND_GRACE_TIME;
                        *frame += 1;
                        if *frame < PREROUND_TRANSITION_TIME / 2 {
                            let amt = *frame as f32 / (PREROUND_TRANSITION_TIME as f32 / 2.0);
                            pixel_camera.offset.y = amt * 2.0;
                        } else if *frame <= PREROUND_TRANSITION_TIME {
                            let amt = ((*frame) - PREROUND_TRANSITION_TIME / 2) as f32
                                / (PREROUND_TRANSITION_TIME as f32 / 2.0);
                            pixel_camera.offset.y = amt * 2.0 - 2.0;
                        }
                        if *frame > max {
                            pixel_camera.offset.y = 0.0;
                            self.state = GameState::InRound;
                        }
                    }
                }
            }

            if self.state.should_draw() {
                self.draw(mouse_x, mouse_y);
            }

            // draw pixel camera to actual screen
            set_default_camera();
            draw_texture_ex(
                &pixel_camera.render_target.as_ref().unwrap().texture,
                horizontal_padding,
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
        window_width: (SCREEN_WIDTH * 1.5) as i32 * 3,
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
