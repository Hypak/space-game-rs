use std::collections::HashMap;

use macroquad::prelude as mq;

use crate::prelude::*;

#[derive(Clone)]
pub struct Base {
    pub game_object: GameObject,
    pub enemy_counts: HashMap<EnemyShipType, u32>,
    pub optimal_distance: f32,
    pub max_distance: f32,
    pub collected: bool,
}

impl Default for Base {
    fn default() -> Self {
        Base { game_object: GameObject::default(), enemy_counts: HashMap::new(), optimal_distance: 100.0, max_distance: 1000.0, collected: false }
    }
}

impl Base {
    pub fn new(position: mq::Vec2, enemy_counts: HashMap<EnemyShipType, u32>, optimal_distance: f32, max_distance: f32) -> Self {
        let game_object = GameObject {
            position,
            radius: 25.0,
            draw_shapes: vec![DrawShape::new_circle_color(mq::RED)],
            ..Default::default()
        };
        Base { game_object, enemy_counts, optimal_distance, max_distance, collected: false }
    }
    pub fn get_enemies(&self) -> Vec<Ship> {
        let mut enemies = vec![];
        for (enemy_type, count) in self.enemy_counts.clone() {
            for i in 0..count {
                let offset = 2.0 * ((i as i32 - count as i32 / 2) as f32) / (count as f32);  // From -1 to 1
                let offset = offset * PI / 4.0;  // From -45 to 45 degrees
                enemies.push(Ship::new_enemy(enemy_type, Some(self.clone()), offset));
            }
        }
        for enemy in &mut enemies {
            let direction = mq::rand::gen_range(-PI, PI);
            enemy.game_object.position = self.game_object.position + self.optimal_distance * mq::Vec2::from_angle(direction);
            enemy.game_object.direction = Direction::new(direction);
        }
        enemies
    }
}
