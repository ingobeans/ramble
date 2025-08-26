use std::sync::LazyLock;

use macroquad::{
    miniquad::{BlendFactor, BlendState, BlendValue, Equation},
    prelude::*,
};

// for screen res i wanted something 16:9 thats also divisble by 16
pub const SCREEN_WIDTH: f32 = 256.0;
pub const SCREEN_HEIGHT: f32 = 144.0;

pub const TILES_HEIGHT: u32 = SCREEN_HEIGHT as u32 / 16;
pub const TILES_WIDTH: u32 = SCREEN_WIDTH as u32 / 16;

pub const OTHER_CHANCE: u8 = 20;

pub const RIGHT: Vec2 = Vec2::new(1.0, 0.0);

/// Select random entry from list. Panics if empty
pub fn select_random<T>(items: &[T]) -> &T {
    &items[rand::gen_range(0, items.len())]
}

pub static WHITE_MATERIAL: LazyLock<Material> = LazyLock::new(|| {
    // to enable transparency!
    let pipeline = PipelineParams {
        alpha_blend: Some(BlendState::new(
            Equation::Add,
            BlendFactor::Value(BlendValue::SourceAlpha),
            BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
        )),
        color_blend: Some(BlendState::new(
            Equation::Add,
            BlendFactor::Value(BlendValue::SourceAlpha),
            BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
        )),
        ..Default::default()
    };
    load_material(
        ShaderSource::Glsl {
            vertex: DEFAULT_VERTEX_SHADER,
            fragment: WHITE_FRAGMENT_SHADER,
        },
        MaterialParams {
            pipeline_params: pipeline,
            ..Default::default()
        },
    )
    .unwrap()
});

pub const WHITE_FRAGMENT_SHADER: &'static str = "#version 100
precision lowp float;

varying vec2 uv;

uniform sampler2D Texture;

void main() {
    if (texture2D(Texture, uv).a > 0.0) {
        gl_FragColor = vec4(255,255,255,255);
    } else {
        gl_FragColor = texture2D(Texture, uv);
    }
}
";

pub const DEFAULT_FRAGMENT_SHADER: &'static str = "#version 100
precision lowp float;

varying vec2 uv;

uniform sampler2D Texture;

void main() {
    gl_FragColor = texture2D(Texture, uv);
}
";

pub const DEFAULT_VERTEX_SHADER: &'static str = "#version 100
precision lowp float;

attribute vec3 position;
attribute vec2 texcoord;

varying vec2 uv;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    uv = texcoord;
}
";
