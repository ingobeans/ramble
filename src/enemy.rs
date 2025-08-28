use macroquad::prelude::*;

use crate::{assets::Assets, projectiles::Projectile, utils::*};

pub enum EnemyMovement {
    /// Enemy chases player
    Chase,
    /// Enemy wanders randomly. Bool is whether it should face the player
    Wander(bool),
    /// Stands still
    Still,
    /// Runs straight forwards
    Fowards,
}

pub enum ProjectileFiring {
    None,
    Cardinally(Projectile, u8),
    Forwards(Projectile, u8),
}

#[expect(dead_code)]
#[derive(PartialEq, Eq, Hash)]
pub enum EnemyTier {
    Light,
    Ranged,
    Heavy,
    Extra,
    Miniboss,
}

pub enum PhaseEndCondition {
    /// Phase lasts indefinitely
    None,
    /// Phase only lasts for a single frame.
    SingleFrame,
    /// Phase lasts for a set number of frames.
    Frames(u32),
    /// Health needs to be under this factor of max.
    HealthUnder(f32),
    /// Phase ends when enemy collides with the player or a wall.
    Collision,
}

pub struct EnemyPhase {
    pub movement: EnemyMovement,
    pub firing: ProjectileFiring,
    pub sprite_x: f32,
    pub sprite_y: f32,
    pub frames: usize,
    pub end: PhaseEndCondition,
}

pub struct EnemyType {
    pub speed: f32,
    pub phases: Vec<EnemyPhase>,
    pub max_health: f32,
}

pub struct Enemy {
    pub ty: &'static EnemyType,
    pub id: usize,
    pub pos: Vec2,
    pub direction: Vec2,
    pub anim_frame: f32,
    /// Used only for [EnemyMovement::Wander]
    pub move_target: Option<Vec2>,
    pub health: f32,
    pub damage_frames: u8,
    pub attack_counter: u8,
    pub phase_index: usize,
    /// Used only when the current phase's end condition is [PhaseEndCondition::Frames]
    pub phase_frame_counter: u32,
}
impl Enemy {
    pub fn new(ty: &'static EnemyType, pos: Vec2, id: usize) -> Self {
        let firing_delay = match &ty.phases.first().unwrap().firing {
            ProjectileFiring::None => 0,
            ProjectileFiring::Cardinally(_, delay) => *delay,
            ProjectileFiring::Forwards(_, delay) => *delay,
        };
        Self {
            ty,
            id,
            pos,
            direction: RIGHT,
            anim_frame: 0.0,
            move_target: None,
            health: ty.max_health,
            damage_frames: 0,
            attack_counter: rand::gen_range(0, firing_delay),
            phase_index: 0,
            phase_frame_counter: 0,
        }
    }
    pub fn get_phase(&self) -> &'static EnemyPhase {
        &self.ty.phases[self.phase_index]
    }
    pub fn draw(&self, assets: &Assets) {
        let x = self.pos.x.floor();
        let y = self.pos.y.floor();
        let draw_params = DrawTextureParams {
            flip_x: self.direction.x < 0.0,
            ..Default::default()
        };
        if self.damage_frames > 0 {
            gl_use_material(&COLOR_MOD_MATERIAL);
        }
        let phase = self.get_phase();
        let anim = (self.anim_frame / 3.0).floor() % phase.frames as f32;
        assets.entities.draw_sprite(
            x,
            y,
            phase.sprite_x + anim,
            phase.sprite_y,
            Some(&draw_params),
        );
        gl_use_default_material();
        // draw health bar
        let width = 12.0;
        let start_x = x - 8.0 + (16.0 - width) / 2.0;
        let start_y = y - 12.0;
        draw_rectangle(start_x, start_y, width, 3.0, Color::from_hex(0x180d2f));
        draw_rectangle(
            start_x,
            start_y,
            self.health / self.ty.max_health * width,
            3.0,
            Color::from_hex(0x47f641),
        );
    }
}
