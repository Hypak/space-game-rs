use macroquad::prelude as mq;

#[derive(Debug, Clone)]
pub enum ShapeType {
    Circle,
    Polygon(u8),
    Line,
}

#[derive(Debug, Clone)]
pub struct DrawShape {
    pub shape_type: ShapeType,
    pub radius_scale: f32,
    pub color: mq::Color,
    pub thickness: f32,
}

const DEFAULT_SHAPE_COLOR: mq::Color = mq::WHITE;
const DEFAULT_CIRCLE_THICKNESS: f32 = 3.0;
const DEFAULT_POLYGON_THICKNESS: f32 = 3.0;
const DEFAULT_LINE_THICKNESS: f32 = 1.5;

impl DrawShape {
    pub fn new_circle() -> Self {
        DrawShape { shape_type: ShapeType::Circle, radius_scale: 1.0, color: DEFAULT_SHAPE_COLOR, thickness: DEFAULT_CIRCLE_THICKNESS }
    }
    pub fn new_polygon(sides: u8) -> Self {
        DrawShape { shape_type: ShapeType::Polygon(sides), radius_scale: 1.0, color: DEFAULT_SHAPE_COLOR, thickness: DEFAULT_POLYGON_THICKNESS }
    }
    pub fn new_line() -> Self {
        DrawShape { shape_type: ShapeType::Line, radius_scale: 1.0, color: DEFAULT_SHAPE_COLOR, thickness: DEFAULT_LINE_THICKNESS }
    }
    pub fn new_circle_color(color: mq::Color) -> Self {
        DrawShape { shape_type: ShapeType::Circle, radius_scale: 1.0, color, thickness: DEFAULT_CIRCLE_THICKNESS }
    }
    pub fn new_polygon_color(sides: u8, color: mq::Color) -> Self {
        DrawShape { shape_type: ShapeType::Polygon(sides), radius_scale: 1.0, color, thickness: DEFAULT_POLYGON_THICKNESS }
    }
    pub fn new_line_color(color: mq::Color) -> Self {
        DrawShape { shape_type: ShapeType::Line, radius_scale: 1.0, color, thickness: DEFAULT_LINE_THICKNESS }
    }
}
