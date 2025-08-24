use macroquad::prelude::*;

use crate::assets::Assets;

pub type Particle = &'static dyn Fn(&ParticleContext, &Assets);

pub struct ParticleContext {
    pub life: u16,
    pub pos: Vec2,
}

//pub const SLASH: Particle = &|ctx, assets| {};
