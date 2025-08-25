use std::sync::Arc;

use macroquad::prelude::*;

use crate::assets::Assets;

pub type Particle = &'static (dyn Fn(&ParticleContext, &Assets) + Send + Sync);

pub struct ParticleContext {
    pub life: u16,
    pub pos: Vec2,
}
