use macroquad::prelude as mq;
use dyn_clone::DynClone;

use crate::prelude::*;

use crate::keybinds;

pub trait WeaponController: DynClone {
    fn is_trying_to_fire (&self, ship: &Ship, game: &Game) -> bool;
}

dyn_clone::clone_trait_object!(WeaponController);

#[derive(Clone)]
pub struct KeyboardWeaponController;

impl WeaponController for KeyboardWeaponController {
    fn is_trying_to_fire (&self, _ship: &Ship, _game: &Game) -> bool {
        mq::is_key_down(keybinds::FIRE_KEY)
    }
}

#[derive(Clone)]
pub struct MouseWeaponController;


impl WeaponController for MouseWeaponController {
    fn is_trying_to_fire (&self, _ship: &Ship, _game: &Game) -> bool {
        mq::is_mouse_button_down(mq::MouseButton::Right)
    }
}

#[derive(Clone)]
pub struct EnemyWeaponController;

impl WeaponController for EnemyWeaponController {
    fn is_trying_to_fire (&self, ship: &Ship, game: &Game) -> bool {
        if let Some(weapon) = ship.weapons.first() {
            let distance = mq::Vec2::distance(ship.game_object.position, game.player.game_object.position);
            let speed = weapon.bullet_speed;
            let time = distance / speed;
            let future_pos = game.player.game_object.position + time * game.player.game_object.velocity;
            let future_offset = future_pos - ship.game_object.position;
            let future_direction = Direction::new_from_vec(future_offset);
            let rotation_direction = Direction::get_shorter_rotation_direction(ship.game_object.direction, future_direction, PI / 18.0);
            return rotation_direction == RotationDirection::None;
        }
        false // Can't fire if there's no weapon
    }
}

#[derive(Clone)]
pub struct EnemyCloneWeaponController;

impl WeaponController for EnemyCloneWeaponController {
    fn is_trying_to_fire (&self, _ship: &Ship, _game: &Game) -> bool {
        true /*
        if let Some(weapon) = ship.weapons.first() {
            let distance = Vec2::distance(ship.game_object.position, game.player.game_object.position);
            let speed_towards_player = ship.game_object.velocity.dot((game.player.game_object.position - ship.game_object.position).normalize());
            let speed = weapon.bullet_speed + speed_towards_player;
            let time = distance / speed;
            let future_pos = game.player.game_object.position + time * game.player.game_object.velocity;
            let future_offset = future_pos - ship.game_object.position;
            let future_direction = Direction::new_from_vec(future_offset);

            // Account for bullets inheriting velocity
            let bullet_vector = weapon.bullet_speed * ship.game_object.direction.get_as_vec() + ship.game_object.velocity;

            let rotation_direction = Direction::get_shorter_rotation_direction(Direction::new_from_vec(bullet_vector), future_direction, PI / 180.0);
            return rotation_direction == RotationDirection::None;
        }
        false // Can't fire if there's no weapon
        */
    }
}
