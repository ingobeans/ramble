use std::sync::LazyLock;

use hashmap_macro::hashmap;
use image::{GenericImageView, Rgba};
use macroquad::prelude::*;

use crate::{
    enemy::{Enemy, EnemyTier, EnemyType},
    utils::*,
};

pub struct DungeonManager {
    pub world: &'static World,
    pub room_index: usize,
}
impl DungeonManager {
    pub fn spawn_room(&self) -> Vec<Enemy> {
        let types = hashmap!(
            EnemyTier::Light => select_random(&self.world.light),
            EnemyTier::Heavy => select_random(&self.world.heavy),
            EnemyTier::Ranged =>  select_random(&self.world.ranged)
        );
        let tiles_width = SCREEN_WIDTH as usize / 16;

        let layout_group_index = self.room_index / 2;
        let layout_group = &LAYOUTS[layout_group_index];
        let layout = &layout_group[rand::gen_range(0, layout_group.len())];
        let mut enemies = Vec::new();
        for (index, value) in layout.iter().enumerate() {
            let Some(value) = value else {
                continue;
            };
            let x = (index % tiles_width) as f32 * 16.0;
            let y = (index / tiles_width) as f32 * 16.0;
            let ty = types[value];
            let enemy = Enemy::new(ty, Vec2::new(x, y), 0);
            enemies.push(enemy);
        }
        enemies
    }
}

pub struct World {
    pub light: Vec<EnemyType>,
    pub heavy: Vec<EnemyType>,
    pub ranged: Vec<EnemyType>,
    pub other: Vec<EnemyType>,
}

pub static LAYOUTS: LazyLock<[Vec<Layout>; 4]> = LazyLock::new(|| {
    let layouts: [&[u8]; 4] = [
        include_bytes!("../assets/layouts/0.png"),
        include_bytes!("../assets/layouts/1.png"),
        include_bytes!("../assets/layouts/2.png"),
        include_bytes!("../assets/layouts/3.png"),
    ];
    let tiles_height = SCREEN_HEIGHT as u32 / 16;
    let tiles_width = SCREEN_WIDTH as u32 / 16;
    std::array::from_fn(|i| {
        let image = image::load_from_memory(layouts[i]).unwrap();
        assert_eq!(image.width(), tiles_width);
        assert!(image.height() % tiles_height == 0);
        let mut vec = Vec::new();
        for layout_index in 0..image.height() / tiles_height {
            vec.push(std::array::from_fn(|i| {
                let y = i as u32 / tiles_width + layout_index * tiles_height;
                let x = i as u32 % tiles_width;
                let value = image.get_pixel(x, y);
                match value {
                    Rgba([0, 0, 0, 255]) => None,
                    Rgba([255, 255, 255, 255]) => Some(EnemyTier::Light),
                    Rgba([0, 0, 255, 255]) => Some(EnemyTier::Heavy),
                    Rgba([255, 255, 0, 255]) => Some(EnemyTier::Ranged),
                    _ => panic!(),
                }
            }));
        }
        vec
    })
});

pub type Layout = [Option<EnemyTier>; SCREEN_WIDTH as usize / 16 * SCREEN_HEIGHT as usize / 16];
