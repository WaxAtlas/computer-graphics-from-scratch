use std::f32::INFINITY;
use std::fs::File;
use std::io::Write;
use std::ops;

struct Canvas {
    height: i32,
    width: i32,
}

impl Canvas {
    fn new(height: i32, width: i32) -> Self {
        Canvas {
            height: height,
            width: width,
        }
    }
}

struct Color {
    r: i32,
    g: i32,
    b: i32,
}

struct Vector {
    x: i32,
    y: i32,
    z: i32,
}

impl Vector {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Vector { x: x, y: y, z: z }
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

impl ops::Mul<i32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: i32) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

struct Sphere {
    center: Vector,
    radius: i32,
    color: Color,
}

const CANVAS: Canvas = Canvas {
    height: 600,
    width: 600,
};
const PROJECTION_PLANE_Z: i32 = 1; // distance from camera to projection plane
const VIEWPORT_SIZE: i32 = 1;

fn canvas_to_viewport(x: i32, y: i32) -> Vector {
    Vector {
        x: VIEWPORT_SIZE / CANVAS.width,
        y: VIEWPORT_SIZE / CANVAS.height,
        z: PROJECTION_PLANE_Z,
    }
}

fn intersect_ray_sphere(origin: Vector, direction: Vector, sphere: Sphere) -> Vec<i32> {
    let r = sphere.radius;
    let co = origin - sphere.center;
}

fn put_pixel(x: i32, y: i32, color: Color) {
    // TODO:
}

fn main() {
    let mut file = File::create("image.ppm").expect("Unable to create file");

    for x in (-CANVAS.width / 2)..(CANVAS.width / 2) {
        for y in (-CANVAS.height / 2)..(CANVAS.height / 2) {
            let direction = canvas_to_viewport(x, y);

            file.write_all(b"").expect("Unable to write data")
        }
    }
}
