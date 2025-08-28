use macroquad::prelude::*;

use crate::assets::Assets;

pub type Particle = &'static (dyn Fn(&ParticleContext, &Assets) + Send + Sync);

pub struct ParticleContext {
    pub life: u16,
    pub pos: Vec2,
    pub origin: Vec2,
}

pub static LIGHT_RAY: Particle = &|ctx, assets| {
    draw_line(ctx.origin.x, ctx.origin.y, ctx.pos.x, ctx.pos.y, 4.0, WHITE);
};
pub static STAR_EXPLOSION: Particle = &|ctx, assets| {
    let anim = (ctx.life as f32 / 15.0 * 3.0).floor();
    assets
        .particles
        .draw_sprite(ctx.pos.x, ctx.pos.y, anim, 48.0 / 16.0, None);
};
