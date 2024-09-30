use macroquad::prelude as mq;

#[derive(Clone)]
pub struct Camera {
    pub position: mq::Vec2,
    pub zoom: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Camera { position: mq::Vec2::ZERO, zoom: 1.0 }
    }
}

impl Camera {
    fn new(position: mq::Vec2, zoom: f32) -> Self {
        Camera { position, zoom,  }
    }
}
