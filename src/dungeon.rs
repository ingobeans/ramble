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
    pub room_in_progress: bool,
}
impl DungeonManager {
    pub fn spawn_room(&self) -> Vec<Enemy> {
        let mut types: std::collections::HashMap<EnemyTier, &EnemyType> = hashmap!(
            EnemyTier::Light => select_random(&self.world.light),
            EnemyTier::Heavy => select_random(&self.world.heavy),
            EnemyTier::Ranged =>  select_random(&self.world.ranged)
        );

        let layout_group_index = self.room_index.min(12);
        let layout_group = &LAYOUTS[layout_group_index];
        let layout = &layout_group[rand::gen_range(0, layout_group.len())];
        let mut enemies = Vec::new();
        let mut last_row = -1.0;
        for (index, value) in layout.iter().enumerate() {
            let Some(value) = value else {
                continue;
            };
            let x = (index % TILES_WIDTH as usize) as f32 * 16.0 + 8.0;
            let y = (index / TILES_WIDTH as usize) as f32 * 16.0 + 8.0;
            if y != last_row {
                types = hashmap!(
                    EnemyTier::Light => select_random(&self.world.light),
                    EnemyTier::Heavy => select_random(&self.world.heavy),
                    EnemyTier::Ranged =>  select_random(&self.world.ranged)
                );
            }
            last_row = y;
            let ty = types[value];
            let enemy = Enemy::new(ty, Vec2::new(x, y), 0);
            enemies.push(enemy);
        }
        if !self.world.other.is_empty() && rand::gen_range(0, 100) < OTHER_CHANCE {
            // spawn an "other" in top left and right corners
            let ty = select_random(&self.world.other);
            let positions = [Vec2::new(8.0, 8.0), Vec2::new(SCREEN_WIDTH - 8.0, 8.0)];
            for position in positions {
                let enemy = Enemy::new(ty, position, 0);
                enemies.push(enemy);
            }
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

pub static LAYOUTS: LazyLock<[Vec<Layout>; 13]> = LazyLock::new(|| {
    let layouts: [&[u8]; _] = [
        include_bytes!("../assets/layouts/0.png"),
        include_bytes!("../assets/layouts/1.png"),
        include_bytes!("../assets/layouts/2.png"),
        include_bytes!("../assets/layouts/3.png"),
        include_bytes!("../assets/layouts/4.png"),
        include_bytes!("../assets/layouts/5.png"),
        include_bytes!("../assets/layouts/6.png"),
        include_bytes!("../assets/layouts/7.png"),
        include_bytes!("../assets/layouts/8.png"),
        include_bytes!("../assets/layouts/9.png"),
        include_bytes!("../assets/layouts/10.png"),
        include_bytes!("../assets/layouts/11.png"),
        include_bytes!("../assets/layouts/12.png"),
    ];

    std::array::from_fn(|i| {
        let image = image::load_from_memory(layouts[i]).unwrap();
        assert_eq!(image.width(), TILES_WIDTH);
        assert!(image.height() % TILES_HEIGHT == 0);
        let mut vec = Vec::new();
        for layout_index in 0..image.height() / TILES_HEIGHT {
            vec.push(std::array::from_fn(|i| {
                let y = i as u32 / TILES_WIDTH + layout_index * TILES_HEIGHT;
                let x = i as u32 % TILES_WIDTH;
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

pub type Layout = [Option<EnemyTier>; TILES_WIDTH as usize * TILES_HEIGHT as usize];
