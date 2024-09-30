use macroquad::prelude as mq;

use crate::prelude::*;

const UPDATE_ENEMY_RADIUS: f32 = 3000.0;

#[derive(Clone)]
pub struct Game {
    pub player: Ship,
    pub camera: Camera,
    pub home_base: GameObject,
    pub bases: Vec<Base>,
    pub enemies: Vec<Ship>,
    pub bullets: Vec<Bullet>,
    pub gameover: bool,
    pub total_enemy_count: usize,
    pub player_speed_multiplier: f32,
    pub enemy_speed_multiplier: f32,
    pub collected_base_count: u32,
}

impl Game {
    pub fn new(level: impl Level, player_level: f32) -> Option<Self> {
        let home_base = GameObject {
            radius: 100.0,
            draw_shapes: vec![DrawShape::new_circle_color(mq::BLUE)],
            ..Default::default()
        };
        let bases = level.get_bases();
        let spawn_regions = level.get_spawn_regions();
        let mut enemies = vec![];
        for base in &bases {
            enemies.extend(base.get_enemies());
        }
        for spawn_region in &spawn_regions {
            enemies.extend(spawn_region.get_enemies());
        }
        let total_enemy_count = enemies.len();
        Some(Game {
            player: Ship::new_player(player_level),
            camera: Camera::default(),
            home_base,
            bases,
            enemies,
            bullets: vec![],
            gameover: false,
            total_enemy_count,
            player_speed_multiplier: 1.0,
            enemy_speed_multiplier: 1.0,
            collected_base_count: 0,
        })
    }

    pub fn update(&mut self, delta_t: f32) {
        let clone = self.clone();
        let mut bullets_to_add = vec![];
        self.player.update(delta_t * self.player_speed_multiplier, &clone, &mut bullets_to_add);
        for enemy in &mut self.enemies {
            if mq::Vec2::distance(self.player.game_object.position, enemy.game_object.position) > UPDATE_ENEMY_RADIUS {
                continue;
            }
            enemy.update(delta_t * self.enemy_speed_multiplier, &clone, &mut bullets_to_add);
            GameObject::kill_if_overlapping(&mut self.player.game_object, &mut enemy.game_object);
        }
        self.bullets.extend(bullets_to_add);
        for bullet in &mut self.bullets {
            if mq::Vec2::distance(self.player.game_object.position, bullet.game_object.position) > UPDATE_ENEMY_RADIUS {
                continue;
            }
            match bullet.team {
                Team::Player => {
                    bullet.update(delta_t * self.player_speed_multiplier);
                    for enemy in &mut self.enemies {
                        GameObject::kill_if_overlapping(&mut bullet.game_object, &mut enemy.game_object);
                    }
                }
                Team::Hostile => {
                    bullet.update(delta_t * self.enemy_speed_multiplier);
                    GameObject::kill_if_overlapping(&mut bullet.game_object, &mut self.player.game_object);
                }
            }
        }
        for base in &mut self.bases {
            if !base.collected && GameObject::is_overlapping(&self.player.game_object, &base.game_object) {
                base.collected = true;
                self.collected_base_count += 1;
            }
            if base.collected {
                base.game_object.position = self.player.game_object.position;
            }
        }
        if self.player.game_object.health_status == HealthStatus::Dead {
            self.gameover = true;
        }
        self.enemies.retain(|enemy| enemy.game_object.health_status != HealthStatus::Dead);
        self.bullets.retain(|bullets| bullets.game_object.health_status != HealthStatus::Dead);
        self.bullets.retain(|bullet| bullet.lifetime_remaining > 0.0);
    }

    pub fn draw(&mut self) {
        self.camera.position = self.player.game_object.position - self.player.game_object.velocity * 0.0;
        self.camera.position -= mq::Vec2::new(mq::screen_width() / 2.0, mq::screen_height() / 2.0) / self.camera.zoom;

        // Draw grid
        let grid_size = 100.0;
        let greater_dimension = f32::max(mq::screen_width(), mq::screen_height());
        let line_half_count = f32::ceil(greater_dimension / grid_size / 2.0 / self.camera.zoom);
        let mut line_half_count = line_half_count as i32;
        if line_half_count >= 192 {
            line_half_count = 0; // Don't render any lines
        }
        let grid_mid_x = f32::round(self.player.game_object.position.x / grid_size) * grid_size;
        let grid_mid_y = f32::round(self.player.game_object.position.y / grid_size) * grid_size;
        let thickness = f32::max(self.camera.zoom, 0.25);
        for x_offset in -line_half_count..=line_half_count {
            let x = grid_mid_x + x_offset as f32 * grid_size - self.camera.position.x;
            let x = x * self.camera.zoom;
            mq::draw_line(x, 0.0, x, mq::screen_height(), thickness, mq::WHITE);
        }
        for y_offset in -line_half_count..=line_half_count {
            let y = grid_mid_y + y_offset as f32 * grid_size - self.camera.position.y;
            let y = y * self.camera.zoom;
            mq::draw_line(0.0, y, mq::screen_width(), y, thickness, mq::WHITE);
        }
        // mq::draw_circle(mq::screen_width() / 2.0, mq::screen_height() / 2.0, 5.0, mq::BLUE);

        for enemy in &self.enemies {
            enemy.game_object.draw(&self.camera);
        }
        for bullet in &self.bullets {
            bullet.game_object.draw(&self.camera);
        }
        self.home_base.draw(&self.camera);
        for base in &self.bases {
            base.game_object.draw(&self.camera);
        }
        self.player.game_object.draw(&self.camera);
    }
}
