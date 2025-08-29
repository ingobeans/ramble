use std::f32::consts::PI;

use hashmap_macro::hashmap;
use macroquad::{miniquad::window::screen_size, prelude::*, rand::gen_range};

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

enum RoundState {
    Active,
    Post(u32, Option<[Item; 3]>),
    Pre(u32),
}
impl RoundState {
    fn should_draw(&self) -> bool {
        match self {
            RoundState::Active | RoundState::Post(_, _) => true,
            RoundState::Pre(frame) => *frame >= 40 / 2,
        }
    }
    fn door_frame(&self) -> f32 {
        match self {
            RoundState::Active => 0.0,
            RoundState::Post(frame, _) => (*frame as f32 / 10.0 * 4.0).floor().min(3.0) * 2.0,
            RoundState::Pre(frame) => {
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
    assets: &'a mut Assets,
    player: Player,
    state: RoundState,
    enemies: Vec<Enemy>,
    dropped_items: Vec<(Vec2, Item)>,
    enemy_id: usize,
    projectiles: Vec<Projectile>,
    dungeon_manager: DungeonManager,
    ui_manager: UiManager,
}
impl<'a> Ramble<'a> {
    fn new(assets: &'a mut Assets) -> Self {
        let mut player = Player::new(Vec2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0));
        player.hand = Some(assets.all_items[3].clone());
        player.helmet = Some(assets.all_items[1].clone());
        player.chestplate = Some(assets.all_items[0].clone());

        Ramble {
            state: RoundState::Post(100, None),
            assets,
            player,
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
    fn give_curse(&mut self, curse: ChaosCurse) {
        let mut lost_items = Vec::new();
        match &curse {
            ChaosCurse::RefillHealth => {
                self.player.regen();
            }
            ChaosCurse::RepairArmor => {
                self.player.repair_armor();
            }
            ChaosCurse::DoubledUnholyDmg => {
                self.player.internal_stats.merge(&Stats {
                    damage_modifiers: hashmap!(DamageType::Unholy=>1.0),
                    ..Default::default()
                });
            }
            ChaosCurse::HalvedHolyDmg => {
                self.player.internal_stats.merge(&Stats {
                    damage_modifiers: hashmap!(DamageType::Holy=>-0.5),
                    ..Default::default()
                });
            }
            ChaosCurse::Gift => {
                let item = select_random(&self.assets.all_items).clone();
                if self.player.inv_slot_free() {
                    self.player.give_item(item);
                } else {
                    let pos =
                        self.player.pos + Vec2::from_angle(rand::gen_range(0.0, PI * 2.0)) * 5.0;
                    self.dropped_items.push((pos, item));
                }
            }
            _ => {}
        }
        self.player.curses.push(curse);
        let inventory_curse_count = self
            .player
            .curses
            .iter()
            .filter(|f| matches!(*f, ChaosCurse::LessInventory))
            .count();

        for (index, item) in self.player.inventory.iter_mut().enumerate() {
            if index < inventory_curse_count * 2
                && let Some(item) = item.take()
            {
                lost_items.push(item);
            }
        }
        for item in lost_items.into_iter() {
            if self.player.inv_slot_free() {
                self.player.give_item(item);
            } else {
                let pos = self.player.pos + Vec2::from_angle(rand::gen_range(0.0, PI * 2.0)) * 5.0;
                self.dropped_items.push((pos, item));
            }
        }
    }
    fn spawn_enemies(&mut self, buffer: &mut Vec<Enemy>) {
        let shield_count = self
            .player
            .curses
            .iter()
            .filter(|f| matches!(*f, ChaosCurse::EnemyShields))
            .count();
        for mut enemy in buffer.drain(..) {
            enemy.id = self.enemy_id;
            enemy.pos.y += 32.0;
            enemy.shield = shield_count as f32 * 5.0;
            self.enemy_id += 1;
            self.enemies.push(enemy);
        }
    }
    fn update(&mut self, mouse_x: f32, mouse_y: f32) {
        let (move_vector, speed) = if self.player.roll.0 == 0 {
            (get_movement_vector(), self.player.stats().move_speed)
        } else {
            (self.player.roll.1, 4.0)
        };
        let max_y = 28.0;

        let bottom_left_corner = Vec2::new(4.0, max_y);
        let top_right_corner = Vec2::new(SCREEN_WIDTH - 4.0, SCREEN_HEIGHT - 8.0);

        self.player.pos =
            (self.player.pos + move_vector * speed).clamp(bottom_left_corner, top_right_corner);
        let door_start_x = TILES_WIDTH / 2 - 1;

        // go to next room
        if let RoundState::Post(_, _) = self.state
            && self.player.pos.y == 28.0
            && (door_start_x as f32 * 16.0 + 4.0..door_start_x as f32 * 16.0 + 28.0)
                .contains(&self.player.pos.x)
            && is_key_down(KeyCode::W)
        {
            self.player.pos = Vec2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT - 16.0);
            self.dropped_items.clear();
            self.enemies.clear();
            self.projectiles.clear();
            self.spawn_enemies(&mut self.dungeon_manager.spawn_room());
            let mut extra_enemies = Vec::new();
            for curse in self.player.curses.iter() {
                match *curse {
                    ChaosCurse::AcidPuddles => {
                        for _ in 0..2 {
                            let mut new = acid_puddle();
                            new.pos = Vec2::new(
                                rand::gen_range(bottom_left_corner.x, top_right_corner.x),
                                rand::gen_range(bottom_left_corner.y + 48.0, top_right_corner.y),
                            );
                            self.projectiles.push(new);
                        }
                    }
                    ChaosCurse::BonusEnemies => {
                        let (types, amt) = if rand::gen_range(0, 100) < 50 {
                            (&self.dungeon_manager.world.other, 2)
                        } else {
                            (&self.dungeon_manager.world.miniboss, 1)
                        };
                        for _ in 0..amt {
                            let enemy = Enemy::new(
                                select_random(&types),
                                Vec2::new(
                                    rand::gen_range(bottom_left_corner.x, top_right_corner.x),
                                    rand::gen_range(
                                        bottom_left_corner.y + 48.0,
                                        top_right_corner.y,
                                    ),
                                ),
                                0,
                            );
                            extra_enemies.push(enemy);
                        }
                    }
                    _ => {}
                }
            }
            self.spawn_enemies(&mut extra_enemies);
            self.dungeon_manager.room_index += 1;
            self.state = RoundState::Pre(0);
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
            // make projectile travel faster if self.player is moving in same direction they're shooting,
            // relative to the projectiles drag. more drag => more affected by players move direction
            projectile.speed +=
                (25.0 * projectile.drag) * (projectile.direction.dot(move_vector).max(0.0));
            self.projectiles.push(projectile);
            self.player.attack_counter = self.player.stats().attack_delay;
        }
        let mut new_projectiles = Vec::new();

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
                        // todo: make projectiles moving faster than 8.0 pixels/frame have their hit scan split in to multiple steps
                        for (k, mut amt) in stats.damage.clone() {
                            if let Some(modifier) = self.player.stats().damage_modifiers.get(&k) {
                                amt *= 1.0 + modifier;
                            }
                            if enemy.shield > 0.0 {
                                enemy.shield = (enemy.shield - amt).max(0.0);
                            } else {
                                enemy.health -= amt;
                            }
                        }
                        enemy.damage_frames = 5;
                        new_projectiles.append(&mut projectile.on_hit());
                        projectile.hit_enemies.push(enemy.id);
                    }
                }
            } else if self.player.can_take_damage() {
                let distance = (self.player.pos - projectile.pos).length();
                if distance <= 4.0 {
                    self.player.damage();
                }
            }
            let old = projectile.pos;
            projectile.pos = projectile.pos.clamp(bottom_left_corner, top_right_corner);
            if projectile.pos != old {
                // projectile collided with wall
                new_projectiles.append(&mut projectile.on_hit());
                return false;
            }

            projectile.life < projectile.lifetime
        });
        self.projectiles.append(&mut new_projectiles);
        let enemy_positions: Vec<Vec2> = self.enemies.iter().map(|f| f.pos).collect();

        self.enemies.retain_mut(|enemy| {
            let player_delta = self.player.pos - enemy.pos;
            enemy.damage_frames = enemy.damage_frames.saturating_sub(1);
            let mut move_direction = Vec2::ZERO;
            let phase = enemy.get_phase();
            // move
            match &phase.movement {
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
                        if *face_player {
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
                            rand::gen_range(bottom_left_corner.x, top_right_corner.x),
                            rand::gen_range(bottom_left_corner.y, top_right_corner.y),
                        ));
                    }
                }
                EnemyMovement::Still => {}
                EnemyMovement::Fowards => {
                    move_direction = enemy.direction;
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
                match &phase.firing {
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
            let mut collision = false;

            let old = enemy.pos;
            enemy.pos = enemy.pos.clamp(bottom_left_corner, top_right_corner);
            if enemy.pos != old {
                collision = true;
            }

            // dmg player on contact
            if player_delta.length() <= 4.0 && self.player.can_take_damage() {
                self.player.damage();
                collision = true;
            }

            // check if phase should change
            let phase_end = match &phase.end {
                PhaseEndCondition::None => false,
                PhaseEndCondition::SingleFrame => true,
                PhaseEndCondition::Collision => collision,
                PhaseEndCondition::Frames(target) => {
                    if enemy.phase_frame_counter >= *target {
                        enemy.phase_frame_counter = 0;
                        true
                    } else {
                        enemy.phase_frame_counter += 1;
                        false
                    }
                }
                PhaseEndCondition::HealthUnder(percent) => {
                    enemy.health / enemy.ty.max_health <= *percent
                }
            };
            if phase_end {
                enemy.phase_index += 1;
                if enemy.phase_index >= enemy.ty.phases.len() {
                    enemy.phase_index = 0;
                }
            }

            enemy.health > 0.0
        });

        if let RoundState::Active = self.state
            && self.enemies.is_empty()
        {
            self.projectiles.retain(|f| f.player_owned);
            self.state = RoundState::Post(
                0,
                Some(std::array::from_fn(|_| {
                    select_random(&self.assets.all_items).clone()
                })),
            )
        }
    }
    fn draw_ui(&mut self, mouse_x: f32, mouse_y: f32) {
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
            if self.player.inv_slot_free() {
                ui::draw_tooltip("press e to pick up", self.assets);
                if is_key_pressed(KeyCode::E) {
                    let item = self.dropped_items.remove(item_under_player.0).1;
                    self.player.give_item(item);
                }
            } else {
                ui::draw_tooltip("inventory full", self.assets);
            }
        }

        self.player.draw(self.assets, mouse_x, mouse_y);

        for projectile in self.projectiles.iter() {
            projectile.draw(self.assets);
        }
    }
    fn handle_item_shop(&mut self) {
        if let RoundState::Post(frame, items) = &mut self.state {
            if *frame <= 40 {
                let y = 88.0;
                let anim = (*frame as f32 / 30.0).min(1.0);
                let x = SCREEN_WIDTH / 2.0;
                draw_ellipse(x, y + 20.0, anim * 48.0, anim * 16.0, 0.0, BLACK);
                if *frame > 30 {
                    let y = y + 14.0 - (*frame - 30) as f32 / 10.0 * 14.0;
                    self.assets.world.sprite_size = 96.0;
                    self.assets.world.draw_sprite(x, y, 0.0, 1.0, None);
                    self.assets.world.sprite_size = 16.0;
                    for index in 0..3 {
                        let x = x - 32.0 + 16.0 * index as f32;

                        if let Some(item) = items.as_ref().map(|f| &f[index]) {
                            ui::draw_slot(
                                Some(item),
                                x + 2.0 - 8.0,
                                y + 1.0,
                                0.0,
                                0.0,
                                self.assets,
                            );

                            // check for distance with a bias against X coordinate
                            if (self.player.pos.x - x).powi(2) * 3.0
                                + (self.player.pos.y - y).powi(2)
                                <= 18.0_f32.powi(2)
                            {
                                if self.player.inv_slot_free() {
                                    ui::draw_tooltip(
                                        "press e to deal with the chaos demon for this item",
                                        self.assets,
                                    );
                                    if is_key_pressed(KeyCode::E) {
                                        let o = items.take().unwrap();
                                        self.player
                                            .give_item(o.into_iter().skip(index).next().unwrap());
                                        let all =
                                            enum_iterator::all::<ChaosCurse>().collect::<Vec<_>>();
                                        let item = *select_random(&all);
                                        *frame = 41;
                                        self.give_curse(item);
                                        return;
                                    }
                                } else {
                                    ui::draw_tooltip("inventory full", self.assets);
                                }
                            }
                        }
                    }
                }
            } else {
                let y = 88.0;
                let frame = *frame - 40;
                let anim = ((30.0 - frame as f32) / 30.0).max(0.0);
                let x = SCREEN_WIDTH / 2.0;
                draw_ellipse(x, y + 20.0, anim * 48.0, anim * 16.0, 0.0, BLACK);
            }
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
        for (index, item) in self.assets.all_items.clone().into_iter().enumerate() {
            self.dropped_items.push((
                Vec2::new(
                    (index as f32 * 14.0 + 20.0) % (SCREEN_WIDTH - 20.0),
                    (index as f32 * 14.0) / (SCREEN_WIDTH - 20.0) + 48.0,
                ),
                item,
            ));
        }

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
                    RoundState::Active => {
                        self.update(mouse_x, mouse_y);
                    }
                    RoundState::Post(frame, _) => {
                        if *frame != 40 {
                            *frame = frame.saturating_add(1);
                        }
                        self.update(mouse_x, mouse_y);
                    }
                    RoundState::Pre(frame) => {
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
                            self.state = RoundState::Active;
                        }
                    }
                }
            }

            if self.player.pos.y > 94.0 {
                self.handle_item_shop()
            }
            if self.state.should_draw() {
                self.draw(mouse_x, mouse_y);
            }
            if self.player.pos.y <= 94.0 {
                self.handle_item_shop()
            }
            self.draw_ui(mouse_x, mouse_y);

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
    let mut assets = Assets::default();
    let mut ramble = Ramble::new(&mut assets);
    ramble.run().await;
}
