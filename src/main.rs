use std::f32::INFINITY;
use std::fs::File;
use std::io::Write;
use std::ops;

struct Canvas {
    height: f32,
    width: f32,
}

impl Canvas {
    fn new(height: f32, width: f32) -> Self {
        Canvas { height, width }
    }
}

struct Color {
    r: f32,
    g: f32,
    b: f32,
}

struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Vector { x, y, z }
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Div<f32> for Vector {
    type Output = Vector;

    fn div(self, rhs: f32) -> Self::Output {
        Vector {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

fn dot(v: &Vector, w: &Vector) -> f32 {
    v.x * w.x + v.y * w.y + v.z * w.z
}

struct Sphere {
    center: Vector,
    radius: f32,
    color: Color,
}

const CANVAS: Canvas = Canvas {
    height: 600.0,
    width: 600.0,
};
const PROJECTION_PLANE_Z: f32 = 1.0; // distance from camera to projection plane
const VIEWPORT_SIZE: f32 = 1.0;

fn canvas_to_viewport(x: f32, y: f32) -> Vector {
    Vector {
        x: x * VIEWPORT_SIZE / CANVAS.width,
        y: y * VIEWPORT_SIZE / CANVAS.height,
        z: PROJECTION_PLANE_Z,
    }
}

fn intersect_ray_sphere(origin: Vector, direction: Vector, sphere: Sphere) -> Vec<f32> {
    let r = sphere.radius;
    let co = origin - sphere.center;

    let a = dot(&direction, &direction);
    let b = 2.0 * dot(&co, &direction);
    let c = dot(&co, &co) - r * r;

    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return vec![INFINITY, INFINITY];
    } else {
        let t1 = -b + f32::sqrt(discriminant) / 2.0 * a;
        let t2 = -b - f32::sqrt(discriminant) / 2.0 * a;
        return vec![t1, t2];
    }
}

fn put_pixel(x: f32, y: f32, color: Color) {
    // TODO:
}

fn main() {
    let mut file = File::create("image.ppm").expect("Unable to create file");

    for x in (-CANVAS.width / 2.0)..(CANVAS.width / 2.0) {
        for y in (-CANVAS.height / 2.0)..(CANVAS.height / 2.0) {
            let direction = canvas_to_viewport(x, y);

            file.write_all(b"").expect("Unable to write data")
        }
    }
}
