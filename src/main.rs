use std::fs::File;
use std::io::Write;
use std::ops;

struct Canvas {
    height: f32,
    width: f32,
}

#[derive(Clone, Copy, Debug, Default)]
struct Color {
    r: f32,
    g: f32,
    b: f32,
}

#[derive(Clone, Copy, Debug, Default)]
struct Vector {
    x: f32,
    y: f32,
    z: f32,
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
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
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

#[derive(Clone)]
struct Sphere {
    center: Vector,
    radius: f32,
    color: Color,
}

// globals

const BACKGROUND_COLOR: Color = Color {
    r: 255.0,
    g: 255.0,
    b: 255.0,
};
const CANVAS: Canvas = Canvas {
    height: 600.0,
    width: 600.0,
};
const CAMERA_POSITION: Vector = Vector {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};
const PROJECTION_PLANE_Z: f32 = 1.0; // distance from camera to projection plane
const VIEWPORT_SIZE: f32 = 1.0;
const SPHERES: [Sphere; 3] = [
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
    },
];

fn canvas_to_viewport(x: f32, y: f32) -> Vector {
    // I am not sure why I need to invert the y axis
    // NOTE: Consider removing inversion later if another output format corrects it
    Vector {
        x: (x * VIEWPORT_SIZE / CANVAS.width),
        y: -(y * VIEWPORT_SIZE / CANVAS.height),
        z: (PROJECTION_PLANE_Z),
    }
}

fn intersect_ray_sphere(origin: &Vector, direction: &Vector, sphere: &Sphere) -> Vec<f32> {
    let r = sphere.radius;
    let co = *origin - sphere.center;

    let a = dot(direction, direction);
    let b = 2.0 * dot(&co, direction);
    let c = dot(&co, &co) - r * r;

    // println!("{:?}, {:?}, {}", co, direction, b);
    // println!("{}, {}, {}", a, b, c);

    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        // println!("discriminant < 0");
        vec![f32::INFINITY, f32::INFINITY]
    } else {
        let t1 = (-b + f32::sqrt(discriminant)) / (2.0 * a);
        let t2 = (-b - f32::sqrt(discriminant)) / (2.0 * a);
        // println!("{}, {}", t1, t2);
        vec![t1, t2]
    }
}

fn trace_ray(origin: &Vector, direction: &Vector, t_min: &f32, t_max: &f32) -> Option<Color> {
    let mut closest_t = f32::INFINITY;
    let mut closest_sphere = None;

    for sphere in SPHERES.iter() {
        let ts = intersect_ray_sphere(origin, direction, sphere);
        if (*t_min < ts[0]) && (ts[0] < *t_max) && (ts[0] < closest_t) {
            closest_t = ts[0];
            closest_sphere = Some(sphere);
        } else if (*t_min < ts[1]) && (ts[1] < *t_max) && (ts[1] < closest_t) {
            closest_t = ts[1];
            closest_sphere = Some(sphere);
        }
    }

    closest_sphere.map(|i| {
        // println!("{:?}", i.color);
        i.color
    })
}

fn put_pixel(x: f32, y: f32, color: &Option<Color>, file: &mut File) {
    let canvas_x = CANVAS.width / 2.0 + x;
    let canvas_y = CANVAS.height / 2.0 - y - 1.0;

    if !(0.0..=CANVAS.width).contains(&canvas_x) || !(0.0..=CANVAS.height).contains(&canvas_y) {
        return;
    }

    match color {
        None => writeln!(
            file,
            "{}, {}, {}",
            BACKGROUND_COLOR.r, BACKGROUND_COLOR.g, BACKGROUND_COLOR.b
        )
        .expect("Error writing background color to file."),
        Some(i) => {
            writeln!(file, "{}, {}, {}", i.r, i.g, i.b).expect("Error writing color to file.")
        }
    };
}

fn main() {
    let mut file = File::create("image.ppm").expect("Unable to create file");

    writeln!(file, "P3\n{} {}\n255", CANVAS.width, CANVAS.height)
        .expect("Error writing header to file.");

    let min_width = -(CANVAS.width / 2.0) as i32;
    let max_width = (CANVAS.width / 2.0) as i32;
    let min_height = -(CANVAS.height / 2.0) as i32;
    let max_height = (CANVAS.height / 2.0) as i32;

    for y in (min_height)..(max_height) {
        for x in (min_width)..(max_width) {
            let direction = canvas_to_viewport(x as f32, y as f32);
            // println!("{:?}", direction);
            let color = trace_ray(&CAMERA_POSITION, &direction, &1.0, &f32::INFINITY);

            put_pixel(x as f32, y as f32, &color, &mut file);
        }
    }
}
