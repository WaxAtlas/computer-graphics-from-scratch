use std::fs;
use std::io;
use std::io::Write;

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

struct Sphere {
    center: i32,
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
    // TODO: this is temporary to stop the LSP from complaining
    let numbers = vec![1, 2];
    numbers
}

fn put_pixel(x: i32, y: i32, color: Color) {
    // TODO:
}

fn main() {
    let mut file = fs::File::create("image.ppm").expect("Unable to create file");

    for x in (-CANVAS.width / 2)..(CANVAS.width / 2) {
        for y in (-CANVAS.height / 2)..(CANVAS.height / 2) {
            let direction = canvas_to_viewport(x, y);

            file.write_all(b"").expect("Unable to write data")
        }
    }
}
