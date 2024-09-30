use macroquad::prelude as mq;

use crate::prelude::*;


#[derive(Clone)]
pub struct Bullet {
    pub game_object: GameObject,
    pub lifetime_remaining: f32,
    pub team: Team,
}

impl Bullet {
    pub fn new_bullet(ship: Ship, bullet_lifetime: f32, bullet_speed: f32) -> Self {
        let fired_from = ship.game_object;
        let direction_vec = fired_from.direction.get_as_vec();
        let position = fired_from.position + fired_from.radius * direction_vec;
        let velocity = fired_from.velocity + bullet_speed * direction_vec;
        let mut circle = DrawShape::new_circle();
        circle.color = mq::BLACK;
        let game_object = GameObject {
            position,
            velocity,
            radius: 2.0,
            draw_shapes: vec![circle],
            direction: Direction::new_from_vec(velocity),
            ..GameObject::default()
        };
        Bullet { game_object, lifetime_remaining: bullet_lifetime, team: ship.team }
    }

    pub fn update(&mut self, delta_t: f32) {
        self.game_object.update(delta_t);
        self.lifetime_remaining -= delta_t;
    }
}
