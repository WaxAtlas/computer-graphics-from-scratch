use crate::Color;
use crate::Vector;

pub struct Canvas {
    pub height: f32,
    pub width: f32,
}

pub struct Light {
    pub light_type: LightType,
    pub intensity: f32,
    pub light_position: Option<Vector>,
}

#[derive(PartialEq, Eq)]
pub enum LightType {
    Ambient,
    Directional,
    Point,
}

#[derive(Clone, Copy)]
pub struct Sphere {
    pub center: Vector,
    pub radius: f32,
    pub color: Color,
}
