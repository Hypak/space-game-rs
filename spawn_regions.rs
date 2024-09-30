use std::collections::HashMap;

use macroquad::prelude as mq;

use crate::prelude::*;

pub struct SpawnRegion {
    enemy_counts: HashMap<EnemyShipType, u32>,
    min_distance: f32,
    max_distance: f32,
}

impl SpawnRegion {
    pub fn new(enemy_counts: HashMap<EnemyShipType, u32>, min_distance: f32, max_distance: f32) -> Self {
        SpawnRegion { enemy_counts, min_distance, max_distance}
    }
    pub fn get_enemies(&self) -> Vec<Ship> {
        let mut enemies = vec![];
        for (enemy_type, count) in self.enemy_counts.clone() {
            for _ in 0..count {
                enemies.push(Ship::new_enemy(enemy_type, None, 0.0));
            }
        }
        for enemy in &mut enemies {
            let direction = mq::rand::gen_range(-PI, PI);
            let square_distance = mq::rand::gen_range(self.min_distance * self.min_distance, self.max_distance * self.max_distance);
            let distance = f32::sqrt(square_distance);
            enemy.game_object.position = distance * mq::Vec2::from_angle(direction);
            enemy.game_object.direction = Direction::new(direction);
        }
        enemies
    }
}
