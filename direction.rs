use std::{f32::consts::PI, ops::{Add, AddAssign, Sub, SubAssign}};

use macroquad::prelude as mq;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Direction {
    direction: f32,
}

impl Direction {
    pub fn new(direction: f32) -> Self {
        let mut new_direction = Direction::default();
        new_direction.set(direction);
        new_direction
    }
    pub fn new_from_vec(vec: mq::Vec2) -> Self {
        let mut new_direction = Direction::default();
        new_direction.set(f32::atan2(vec.y, vec.x));
        new_direction
    }
    pub fn get(self) -> f32 {
        self.direction
    }
    pub fn get_as_degrees(self) -> f32 {
        self.direction * 180.0 / PI
    }
    pub fn get_as_vec(self) -> mq::Vec2 {
        mq::Vec2::from_angle(self.direction)
    }
    pub fn set(&mut self, direction: f32) {
        self.direction = direction;
        while self.direction > PI {
            self.direction -= 2.0 * PI;
        }
        while self.direction < -PI {
            self.direction += 2.0 * PI;
        }
    }
    pub fn add_f32(&mut self, increment: f32) {
        self.set(self.direction + increment);
    }
    pub fn get_shorter_rotation_direction(current_direction: Self, target_direction: Self, epsilon: f32) -> RotationDirection {
        let mut difference = (target_direction - current_direction).direction;

        if difference.abs() < epsilon{
            return RotationDirection::None;
        }

        if difference > PI {
            difference -= 2.0 * PI;
        } else if difference < -PI {
            difference += 2.0 * PI;
        }
        RotationDirection::from_bool(false, difference > 0.0)
    }
}

impl Add for Direction {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.direction + other.direction)
    }
}

impl Sub for Direction {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.direction - other.direction)
    }
}

impl AddAssign for Direction {
    fn add_assign(&mut self, other: Self) {
        self.set(self.direction + other.direction);
    }
}

impl SubAssign for Direction {
    fn sub_assign(&mut self, other: Self) {
        self.set(self.direction - other.direction);
    }
}

#[derive(PartialEq, Eq)]
pub enum RotationDirection {
    Left,
    Right,
    None,
}

#[allow(dead_code)]
impl RotationDirection {
    pub fn to_f32(&self) -> f32 {
        match self {
            RotationDirection::Left => -1.0,
            RotationDirection::Right => 1.0,
            RotationDirection::None => 0.0,
        }
    }
    pub fn from_f32(x: f32) -> Self {
        match x {
            -1.0 => RotationDirection::Left,
            1.0 => RotationDirection::Right,
            0.0 => RotationDirection::None,
            _ => panic!("Invalid input, should be 0.0, -1.0, or 1.0"),
        }
    }
    pub fn from_i32(x: i32) -> Self {
        match x {
            -1 => RotationDirection::Left,
            1 => RotationDirection::Right,
            0 => RotationDirection::None,
            _ => panic!("Invalid input, should be 0.0, -1.0, or 1.0"),
        }
    }
    pub fn from_bool(is_none: bool, is_right: bool) -> Self {
        match is_none {
            true => RotationDirection::None,
            false => match is_right {
                true => RotationDirection::Right,
                false => RotationDirection::Left,
            }
        }
    }
}
