pub mod controller;
pub mod enemy_ship_type;

use macroquad::prelude as mq;

use crate::prelude::*;

use controller::{EmptyShipController, EnemyShipController};

#[derive(Clone)]
pub struct Ship {
    pub game_object: GameObject,
    pub ship_controller: Box<dyn ShipController>,
    pub thrust: f32,
    pub rotate_speed: f32,
    pub weapons: Vec<Weapon>,
    pub team: Team,
}

impl Default for Ship {
    fn default() -> Self {
        Ship { game_object: GameObject::default(), ship_controller: Box::new(EmptyShipController), thrust: 0.0, rotate_speed: 0.0, weapons: vec![], team: Team::Player }
    }
}

impl Ship {
    pub fn new_player(level: f32) -> Self {
        let circle = DrawShape::new_circle_color(mq::BLUE);
        let triangle = DrawShape::new_polygon_color(3, mq::DARKBLUE);
        let mut line = DrawShape::new_line_color(mq::SKYBLUE);
        line.radius_scale = 5000.0;
        let game_object = GameObject {
            radius: 10.0,
            friction_multiplier: 0.22,
            friction_constant: 15.0,
            draw_shapes: vec![circle, triangle, line],
            // health_status: HealthStatus::Invulnerable,
            ..Default::default()
        };
        Ship {
            game_object,
            ship_controller: Box::new(controller::MouseShipController),
            thrust: 200.0 + 20.0 * level, rotate_speed: 0.6 * PI + 0.1 * level,
            weapons: vec![Weapon::new_player_weapon(level)],
            team: Team::Player,
        }
    }
    pub fn new_enemy(enemy_type: EnemyShipType, base: Option<Base>, player_direction_offset: f32) -> Self {
        match enemy_type {
            EnemyShipType::UltraLowFriction => Ship::new_enemy_ultra_low_friction(base, player_direction_offset),
            EnemyShipType::LowFriction => Ship::new_enemy_low_friction(base, player_direction_offset),
            EnemyShipType::HighFriction => Ship::new_enemy_hi_friction(base, player_direction_offset),
            EnemyShipType::Slow => Ship::new_enemy_slow(base, player_direction_offset),
            EnemyShipType::Shoot => Ship::new_enemy_shoot(base, player_direction_offset),
            EnemyShipType::Turret => Ship::new_enemy_turret(base, player_direction_offset),
            EnemyShipType::Sniper => Ship::new_enemy_sniper(base, player_direction_offset),
            EnemyShipType::Glider => Ship::new_enemy_glider(base, player_direction_offset),
            EnemyShipType::Clone => Ship::new_player_clone(base, player_direction_offset),
        }
    }

    fn new_player_clone(base: Option<Base>, player_direction_offset: f32) -> Self {
        let circle = DrawShape::new_circle_color(mq::BLUE);
        let triangle = DrawShape::new_polygon_color(3, mq::DARKBLUE);
        let mut line = DrawShape::new_line_color(mq::SKYBLUE);
        line.radius_scale = 5000.0;
        let game_object = GameObject {
            radius: 10.0,
            friction_multiplier: 0.22,
            friction_constant: 15.0,
            draw_shapes: vec![circle, triangle, line],
            ..Default::default()
        };
        Ship {
            game_object,
            ship_controller: Box::new(EnemyShipController::new(base, 1500.0, 0.0 * player_direction_offset)),
            thrust: 270.0, rotate_speed: 1.0 * PI,
            weapons: vec![Weapon::new_enemy_clone_weapon()],
            team: Team::Hostile,
        }
    }

    fn new_enemy_ultra_low_friction(base: Option<Base>, player_direction_offset: f32) -> Self {
        let circle = DrawShape::new_circle_color(mq::BEIGE);
        let square = DrawShape::new_polygon_color(7, mq::BEIGE);
        let game_object = GameObject {
            radius: 10.0, friction_multiplier: 0.95, friction_constant: 10.0,
            draw_shapes: vec![circle, square],
            ..Default::default()
        };
        Ship {
            game_object,
            ship_controller: Box::new(EnemyShipController::new(base, 1500.0, 1.0 * player_direction_offset)),
            thrust: 50.0, rotate_speed: 0.4,
            weapons: vec![],
            team: Team::Hostile,
        }
    }

    fn new_enemy_low_friction(base: Option<Base>, player_direction_offset: f32) -> Self {
        let circle = DrawShape::new_circle_color(mq::RED);
        let square = DrawShape::new_polygon_color(4, mq::RED);
        let game_object = GameObject {
            radius: 15.0, friction_multiplier: 0.8, friction_constant: 10.0,
            draw_shapes: vec![circle, square],
            ..Default::default()
        };
        Ship {
            game_object,
            ship_controller: Box::new(EnemyShipController::new(base, 1200.0, 0.5 * player_direction_offset)),
            thrust: 100.0, rotate_speed: 0.8,
            weapons: vec![],
            team: Team::Hostile,
        }
    }

    fn new_enemy_hi_friction(base: Option<Base>, player_direction_offset: f32) -> Self {
        let circle = DrawShape::new_circle_color(mq::GREEN);
        let hex = DrawShape::new_polygon_color(6, mq::DARKGREEN);
        let game_object = GameObject {
            radius: 25.0, friction_multiplier: 0.15, friction_constant: 10.0,
            draw_shapes: vec![circle, hex],
            ..Default::default()
        };
        Ship {
            game_object,
            ship_controller: Box::new(EnemyShipController::new(base, 1000.0, 0.5 * player_direction_offset)),
            thrust: 300.0, rotate_speed: 1.5,
            weapons: vec![],
            team: Team::Hostile,
        }
    }

    fn new_enemy_slow(base: Option<Base>, player_direction_offset: f32) -> Self {
        let circle = DrawShape::new_circle_color(mq::BROWN);
        let hex = DrawShape::new_polygon_color(5, mq::DARKBROWN);
        let game_object = GameObject {
            radius: 35.0, friction_multiplier: 0.05, friction_constant: 10.0,
            draw_shapes: vec![circle, hex],
            ..Default::default()
        };
        Ship {
            game_object,
            ship_controller: Box::new(EnemyShipController::new(base, 800.0, 1.0 * player_direction_offset)),
            thrust: 300.0, rotate_speed: PI,
            weapons: vec![],
            team: Team::Hostile,
        }
    }

    fn new_enemy_shoot(base: Option<Base>, player_direction_offset: f32) -> Self {
        let circle = DrawShape::new_circle_color(mq::BLUE);
        let tri = DrawShape::new_polygon_color(3, mq::DARKBLUE);
        let game_object = GameObject {
            radius: 10.0, friction_multiplier: 0.22, friction_constant: 15.0,
            draw_shapes: vec![circle, tri],
            ..Default::default()
        };
        let weapon = Weapon::new_enemy_clone_weapon();
        Ship {
            game_object,
            ship_controller: Box::new(EnemyShipController::new(base, 1200.0, 0.0 * player_direction_offset)),
            thrust: 300.0, rotate_speed: PI / 6.0,
            weapons: vec![weapon],
            team: Team::Hostile,
        }
    }

    fn new_enemy_turret(base: Option<Base>, player_direction_offset: f32) -> Self {
        let circle = DrawShape::new_circle_color(mq::GOLD);
        let tri = DrawShape::new_polygon_color(3, mq::GOLD);
        let game_object = GameObject {
            radius: 25.0, friction_multiplier: 0.22, friction_constant: 15.0,
            draw_shapes: vec![circle, tri],
            ..Default::default()
        };
        let mut weapon = Weapon::new_enemy_clone_weapon();
        weapon.bullet_lifetime = 8.0;
        weapon.reload_time = 1.2;
        weapon.bullet_speed = 250.0;
        Ship {
            game_object,
            ship_controller: Box::new(EnemyShipController::new(base, 1200.0, 0.0 * player_direction_offset)),
            thrust: 30.0, rotate_speed: PI,
            weapons: vec![weapon],
            team: Team::Hostile,
        }
    }

    fn new_enemy_sniper(base: Option<Base>, player_direction_offset: f32) -> Self {
        let circle = DrawShape::new_circle_color(mq::ORANGE);
        let tri = DrawShape::new_polygon_color(3, mq::ORANGE);
        let mut line = DrawShape::new_line_color(mq::ORANGE);
        line.radius_scale = 2000.0;
        let game_object = GameObject {
            radius: 20.0, friction_multiplier: 0.22, friction_constant: 15.0,
            draw_shapes: vec![circle, tri, line],
            ..Default::default()
        };
        let mut weapon = Weapon::new_enemy_clone_weapon();
        weapon.bullet_lifetime = 8.0;
        weapon.reload_time = 10.0;
        weapon.bullet_speed = 350.0;
        Ship {
            game_object,
            ship_controller: Box::new(EnemyShipController::new(base, 1500.0, 0.0 * player_direction_offset)),
            thrust: 90.0, rotate_speed: PI / 6.0,
            weapons: vec![weapon],
            team: Team::Hostile,
        }
    }

    fn new_enemy_glider(base: Option<Base>, player_direction_offset: f32) -> Self {
        let circle = DrawShape::new_circle_color(mq::MAROON);
        let tri = DrawShape::new_polygon_color(3, mq::MAROON);
        let game_object = GameObject {
            radius: 20.0, friction_multiplier: 0.92, friction_constant: 10.0,
            draw_shapes: vec![circle, tri],
            ..Default::default()
        };
        let mut weapon = Weapon::new_enemy_clone_weapon();
        weapon.bullet_lifetime = 10.0;
        weapon.reload_time = 1.2;
        weapon.bullet_speed = 100.0;
        Ship {
            game_object,
            ship_controller: Box::new(EnemyShipController::new(base, 1500.0, 1.0 * player_direction_offset)),
            thrust: 50.0, rotate_speed: PI / 2.0,
            weapons: vec![weapon],
            team: Team::Hostile,
        }
    }

    pub fn update(&mut self, delta_t: f32, game: &Game, bullets_to_add: &mut Vec<Bullet>) {
        let is_thrusting = self.ship_controller.is_thrusting(self, game);
        if is_thrusting {
            self.game_object.velocity += self.game_object.direction.get_as_vec() * self.thrust * delta_t;
        }
        let rotation = self.ship_controller.get_rotation(self, game);
        let rotation = rotation.to_f32();
        self.game_object.direction.add_f32(rotation * self.rotate_speed * delta_t);
        self.game_object.update(delta_t);
        let clone = self.clone();
        for weapon in &mut self.weapons {
            weapon.update(delta_t, &clone, game, bullets_to_add);
        }
    }
}
