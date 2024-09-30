pub mod draw_shape;
pub mod health_status;

use macroquad::prelude as mq;
use crate::prelude::*;

use health_status::HealthStatus;
use draw_shape::{DrawShape, ShapeType};

#[derive(Debug, Clone)]
pub struct GameObject {
    pub position: mq::Vec2,
    pub velocity: mq::Vec2,
    pub radius: f32,
    pub friction_constant: f32,   // 0 is no friction
    pub friction_multiplier: f32, // 1 is no fricion
    pub direction: Direction,
    pub draw_shapes: Vec<DrawShape>,
    pub health_status: HealthStatus,
}

impl Default for GameObject {
    fn default() -> Self {
        GameObject {
            position: mq::Vec2::ZERO, velocity: mq::Vec2::ZERO, radius: 1.0,
            friction_constant: 0.0, friction_multiplier: 1.0, direction: Direction::new(0.0),
            draw_shapes: vec![], health_status: HealthStatus::Alive,
        }
    }
}


impl GameObject {
    fn randomize_position(&mut self, min_distance: f32, max_distance: f32) {
        let direction = mq::rand::gen_range(-PI, PI);
        let square_distance = mq::rand::gen_range(min_distance * min_distance, max_distance * max_distance);
        let distance = f32::sqrt(square_distance);
        self.position = distance * mq::Vec2::from_angle(direction);
        self.direction = Direction::new(direction);
    }

    pub fn update(&mut self, delta_t: f32) {
        let mut new_velocity = self.velocity * self.friction_multiplier.powf(delta_t);
        let friction_this_frame = self.friction_constant * delta_t;
        if friction_this_frame >= new_velocity.length() {
            new_velocity = mq::Vec2::ZERO;
        } else {
            new_velocity -= self.velocity.normalize() * friction_this_frame;
        }
        let average_velocity = (self.velocity + new_velocity) / 2.0;
        self.position += average_velocity * delta_t;
        self.velocity = new_velocity;
    }

    pub fn draw(&self, camera: &Camera) {
        for shape in &self.draw_shapes {
            let relative_pos = (self.position - camera.position) * camera.zoom;
            let x = relative_pos.x;
            let y = relative_pos.y;
            let thickness = f32::max(1.0, shape.thickness * camera.zoom);
            match shape.shape_type {
                ShapeType::Circle => mq::draw_circle_lines(x, y, self.radius * shape.radius_scale * camera.zoom, thickness, shape.color),
                ShapeType::Polygon(sides) => mq::draw_poly_lines(
                    x, y, sides, self.radius * shape.radius_scale * camera.zoom,
                    self.direction.get_as_degrees(), thickness, shape.color
                ),
                ShapeType::Line => {
                    let bullet_speed = 500.0; // TODO: Change dynamically
                    let adjusted_direction_vec = bullet_speed * self.direction.get_as_vec() + self.velocity;
                    let adjusted_direction_vec = adjusted_direction_vec.normalize();
                    let start = relative_pos + self.radius * self.direction.get_as_vec() * camera.zoom;
                    let _end = start + shape.radius_scale * self.direction.get_as_vec() * camera.zoom;
                    let end_adjusted = start + shape.radius_scale * adjusted_direction_vec * camera.zoom;
                    // mq::draw_line(start.x, start.y, end.x, end.y, shape.thickness, shape.color);
                    mq::draw_line(start.x, start.y, end_adjusted.x, end_adjusted.y, thickness, shape.color);
                }
            }
        }
    }

    pub fn is_overlapping(x: &Self, y: &Self) -> bool {
        let distance = x.position.distance(y.position);
        distance <= x.radius + y.radius
    }

    pub fn kill_if_overlapping(x: &mut Self, y: &mut Self) {
        if GameObject::is_overlapping(x, y) {
            x.health_status = HealthStatus::kill_if_alive(x.health_status);
            y.health_status = HealthStatus::kill_if_alive(y.health_status);
        }
    }
}
