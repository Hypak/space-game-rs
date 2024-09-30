use crate::prelude::*;
#[allow(unused_imports)]
use controller::{WeaponController, MouseWeaponController, KeyboardWeaponController, EnemyWeaponController, EnemyCloneWeaponController};

pub mod controller;
pub mod bullet;

#[derive(Clone)]
pub struct Weapon {
    pub weapon_controller: Box<dyn WeaponController>,
    pub reload_time: f32,
    pub time_until_reloaded: f32,
    pub bullet_lifetime: f32,
    pub bullet_speed: f32,
}

impl Weapon {
    pub fn new_player_weapon(level: f32) -> Self {
        let bullet_speed = 300.0 + 50.0 * level;
        let range = 600.0;
        Weapon {
            weapon_controller: Box::new(MouseWeaponController),
            reload_time: 4.0 / (level + 5.0),
            time_until_reloaded: 0.0,
            bullet_lifetime: range / bullet_speed,
            bullet_speed,
        }
    }

    fn new_enemy_weapon() -> Self {
        Weapon {
            weapon_controller: Box::new(EnemyWeaponController),
            reload_time: 2.5,
            time_until_reloaded: 0.0,
            bullet_lifetime: 5.0,
            bullet_speed: 200.0,
        }
    }

    pub fn new_enemy_clone_weapon() -> Self {
        Weapon {
            weapon_controller: Box::new(EnemyCloneWeaponController),
            reload_time: 1.0,
            time_until_reloaded: 0.0,
            bullet_lifetime: 5.0,
            bullet_speed: 200.0,
        }
    }

    pub fn update(&mut self, delta_t: f32, ship: &Ship, game: &Game, bullets_to_add: &mut Vec<Bullet>) {
        self.time_until_reloaded -= delta_t;
        if self.weapon_controller.is_trying_to_fire(ship, game) {
            while self.time_until_reloaded <= 0.0 {
                self.time_until_reloaded += self.reload_time;
                bullets_to_add.push(Bullet::new_bullet(ship.clone(), self.bullet_lifetime, self.bullet_speed));
            }
        } else {
            self.time_until_reloaded = self.time_until_reloaded.max(0.0);
        }
    }
}
