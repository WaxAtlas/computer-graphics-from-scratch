use crate::Canvas;
use crate::Color;
use crate::Sphere;
use crate::Vector;

use crate::structs;
use structs::Light;
use structs::LightType;

pub const BACKGROUND_COLOR: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
};
pub const CANVAS: Canvas = Canvas {
    height: 600.0,
    width: 600.0,
};
pub const CAMERA_POSITION: Vector = Vector {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

pub const LIGHTS: [Light; 3] = [
    Light {
        light_type: LightType::Ambient,
        intensity: 0.2,
        light_position: None,
    },
    Light {
        light_type: LightType::Point,
        intensity: 0.6,
        light_position: Some(Vector {
            x: 2.0,
            y: 1.0,
            z: 0.0,
        }),
    },
    Light {
        light_type: LightType::Directional,
        intensity: 0.2,
        light_position: Some(Vector {
            x: 1.0,
            y: 4.0,
            z: 4.0,
        }),
    },
];

pub const SPHERES: [Sphere; 4] = [
    Sphere {
        center: Vector {
            x: 0.0,
            y: -1.0,
            z: 3.0,
        },
        radius: 1.0,
        color: Color {
            r: 255.0,
            g: 0.0,
            b: 0.0,
        },
        specular: 500.0,
        reflective: 0.2,
    },
    Sphere {
        center: Vector {
            x: -2.0,
            y: 0.0,
            z: 4.0,
        },
        radius: 1.0,
        color: Color {
            r: 0.0,
            g: 255.0,
            b: 0.0,
        },
        specular: 10.0,
        reflective: 0.4,
    },
    Sphere {
        center: Vector {
            x: 2.0,
            y: 0.0,
            z: 4.0,
        },
        radius: 1.0,
        color: Color {
            r: 0.0,
            g: 0.0,
            b: 255.0,
        },
        specular: 500.0,
        reflective: 0.3,
    },
    Sphere {
        center: Vector {
            x: 0.0,
            y: -5001.0,
            z: 0.0,
        },
        radius: 5000.0,
        color: Color {
            r: 255.0,
            g: 255.0,
            b: 0.0,
        },
        specular: 1000.0,
        reflective: 0.5,
    },
];
