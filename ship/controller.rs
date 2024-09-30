use std::ops::Sub;
use crate::prelude::*;

use macroquad::prelude as mq;
use dyn_clone::DynClone;

use crate::keybinds;

pub trait ShipController: DynClone {
    fn is_thrusting (&self, ship: &Ship, game: &Game) -> bool;
    fn get_rotation(&self, ship: &Ship, game: &Game) -> RotationDirection;
}

dyn_clone::clone_trait_object!(ShipController);

#[derive(Clone)]
pub struct EmptyShipController;

impl ShipController for EmptyShipController {
    fn is_thrusting (&self, _ship: &Ship, _game: &Game) -> bool {
        false
    }

    fn get_rotation(&self, _ship: &Ship, _game: &Game) -> RotationDirection {
        RotationDirection::None
    }
}

#[derive(Clone)]
pub struct KeyboardShipController;

impl ShipController for KeyboardShipController {
    fn is_thrusting (&self, _object: &Ship, _game: &Game) -> bool {
        mq::is_key_down(keybinds::THRUST_KEY)
    }

    fn get_rotation(&self, _object: &Ship, _game: &Game) -> RotationDirection {
        let mut rotate = 0;
        if mq::is_key_down(keybinds::LEFT_KEY) {
            rotate -= 1;
        }
        if mq::is_key_down(keybinds::RIGHT_KEY) {
            rotate += 1;
        }
        RotationDirection::from_i32(rotate)
    }
}

#[derive(Clone)]
pub struct MouseShipController;

impl ShipController for MouseShipController {
    fn is_thrusting (&self, _object: &Ship, _game: &Game) -> bool {
        mq::is_mouse_button_down(mq::MouseButton::Left)
    }

    fn get_rotation(&self, ship: &Ship, _game: &Game) -> RotationDirection {
        let middle = mq::vec2(mq::screen_width(), mq::screen_height()) / 2.0;
        let mouse_pos = mq::vec2(mq::mouse_position().0, mq::mouse_position().1);
        let mouse_from_mid = mouse_pos - middle;
        let target_direction = Direction::new_from_vec(mouse_from_mid);
        Direction::get_shorter_rotation_direction(ship.game_object.direction, target_direction, PI / 180.0)
    }
}

#[derive(Clone)]
pub struct EnemyShipController {
    base: Option<Base>,
    player_persue_distance: f32,
    player_direction_offset: f32,
    epsilon: f32,
}

impl EnemyShipController {
    pub fn new(base: Option<Base>, player_persue_distance: f32, player_direction_offset: f32) -> Self {
        EnemyShipController {
            base,
            player_persue_distance,
            player_direction_offset,
            ..Default::default()
        }
    }
    fn get_target(&self, ship: &Ship, game: &Game) -> (Option<mq::Vec2>, Option<mq::Vec2>) {
        let ship_pos = ship.game_object.position;
        let player_pos = game.player.game_object.position;
        let player_dist = mq::Vec2::distance(ship_pos, player_pos);
        if let Some(base) = &self.base {
            let base_pos = base.game_object.position;
            let base_dist = mq::Vec2::distance(ship_pos, base_pos);
            let player_base_dist = mq::Vec2::distance(player_pos, base_pos);
            if player_dist < self.player_persue_distance && player_base_dist < base.max_distance {
                return (Some(player_pos), Some(game.player.game_object.velocity));
            }
            if base_dist > base.optimal_distance + self.epsilon {
                return (Some(base_pos), None);
            }
            if base_dist < base.optimal_distance - self.epsilon {
                let direction_vec_from_base = (ship.game_object.position - base_pos).normalize();
                // Returns the nearest point on the circle of radius `optimal_base_distance` centered on the base
                return (Some(base_pos + direction_vec_from_base * base.optimal_distance), None);
            }
        }
        if player_dist < self.player_persue_distance {
            return (Some(player_pos), Some(game.player.game_object.velocity));
        }
        (None, None)
    }
}

impl Default for EnemyShipController {
    fn default() -> Self {
        EnemyShipController { base: None, player_persue_distance: 600.0, player_direction_offset: 0.0, epsilon: 1.0 }
    }
}

impl ShipController for EnemyShipController {
    fn is_thrusting (&self, ship: &Ship, game: &Game) -> bool {
        let (target_pos, _) = self.get_target(ship, game);
        if let Some(target) = target_pos {
            let dot = ship.game_object.direction.get_as_vec().dot(target - ship.game_object.position);
            return dot > 0.0;
        }
        false
    }

    fn get_rotation(&self, ship: &Ship, game: &Game) -> RotationDirection {
        let (target_pos, opt_target_velocity) = self.get_target(ship, game);
        let prediction_speed;
        if let Some(weapon) = ship.weapons.first() {
            prediction_speed = weapon.bullet_speed;
        } else {
            // TODO: change this to ship's speed
            prediction_speed = 200.0;
        }
        if let Some(target_pos) = target_pos {
            let target_velocity;
            if let Some(_target_velocity) = opt_target_velocity {
                target_velocity = _target_velocity;
            } else {
                target_velocity = mq::Vec2::ZERO;
            }
            let target_distance = mq::Vec2::distance(ship.game_object.position, target_pos);
            let time = target_distance / prediction_speed;
            let predicted_position = target_pos + target_velocity * time;
            let offset = predicted_position.sub(ship.game_object.position);
            let mut target_direction = Direction::new_from_vec(offset);
            target_direction.add_f32(self.player_direction_offset);
            return Direction::get_shorter_rotation_direction(ship.game_object.direction, target_direction, PI / 180.0);
        }
        RotationDirection::None
    }
}

#[derive(Clone)]
pub struct EnemyCloneShipController;

impl EnemyCloneShipController {
    fn get_closest_bullet_threat(ship: &Ship, game: &Game) -> Option<Bullet> {
        let mut closest_distance = f32::INFINITY;
        let mut closest_bullet = None;
        for bullet in &game.bullets {
            if bullet.team == ship.team {
                continue;
            }
            let vec_from_bullet = ship.game_object.position - bullet.game_object.position;
            let direction = Direction::new_from_vec(vec_from_bullet);
            let difference = direction - bullet.game_object.direction;
            if difference.get().abs() < PI / 18.0 {
                let distance = mq::Vec2::distance(ship.game_object.position, bullet.game_object.position);
                if distance < closest_distance {
                    closest_distance = distance;
                    closest_bullet = Some(bullet);
                }
            }
        }
        closest_bullet.cloned()
    }
    fn is_player_threat(ship: &Ship, game: &Game) -> bool {
        let vec_from_player = ship.game_object.position - game.player.game_object.position;
        let direction = Direction::new_from_vec(vec_from_player);
        let difference = direction - game.player.game_object.direction;
        difference.get().abs() < PI / 4.0
    }
}

impl ShipController for EnemyCloneShipController {
    fn is_thrusting (&self, ship: &Ship, game: &Game) -> bool {
        let closest_bullet = EnemyCloneShipController::get_closest_bullet_threat(ship, game);
        let dot = ship.game_object.direction.get_as_vec().dot(game.player.game_object.position - ship.game_object.position);
        closest_bullet.is_some() || dot > 0.0 || EnemyCloneShipController::is_player_threat(ship, game)
    }

    fn get_rotation(&self, ship: &Ship, game: &Game) -> RotationDirection {
        let weapon = ship.weapons.first().expect("pls");
        let closest_bullet = EnemyCloneShipController::get_closest_bullet_threat(ship, game);
        let target_direction;
        if let Some(closest_bullet) = closest_bullet {
            let speed = closest_bullet.game_object.velocity.length();
            let distance = mq::Vec2::distance(ship.game_object.position, closest_bullet.game_object.position);
            let time = distance / speed;
            let future_ship_pos = ship.game_object.position + 0.5 * ship.game_object.velocity * time;
            let direction_from_bullet = Direction::new_from_vec(future_ship_pos - closest_bullet.game_object.position);
            let bullet_miss_angle = direction_from_bullet - closest_bullet.game_object.direction;
            target_direction = if bullet_miss_angle.get() > 0.0 {
                closest_bullet.game_object.direction + Direction::new(PI / 2.0)
            } else {
                closest_bullet.game_object.direction - Direction::new(PI / 2.0)
            };
        } else if EnemyCloneShipController::is_player_threat(ship, game) && weapon.time_until_reloaded > weapon.reload_time / 2.0 {
            let direction_from_player = Direction::new_from_vec(ship.game_object.position - game.player.game_object.position);
            let player_miss_angle = direction_from_player - game.player.game_object.direction;
            target_direction = if player_miss_angle.get() > 0.0 {
                game.player.game_object.direction + Direction::new(PI * 0.75)
            } else {
                game.player.game_object.direction - Direction::new(PI * 0.75)
            };
        } else {
            let mut player_pos = game.player.game_object.position;
            let mut lookahead_seconds = 0.0;
            for _ in 0..3 {
                let player_dist = mq::Vec2::distance(ship.game_object.position, player_pos);
                let speed_towards_player = ship.game_object.velocity.dot((player_pos - ship.game_object.position).normalize());
                let speed = weapon.bullet_speed + speed_towards_player;
                lookahead_seconds = player_dist / speed;
                player_pos = game.player.game_object.position + lookahead_seconds * game.player.game_object.velocity;
            }
            let future_self_pos = ship.game_object.position + lookahead_seconds * ship.game_object.velocity;
            let offset = player_pos.sub(future_self_pos);
            target_direction = Direction::new_from_vec(offset);

        }
        Direction::get_shorter_rotation_direction(ship.game_object.direction, target_direction, PI / 180.0)
    }
}
