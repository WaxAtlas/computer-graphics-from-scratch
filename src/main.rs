use std::fs::File;
use std::io::Write;

// my mods
mod structs;

use structs::Canvas;
use structs::Light;
use structs::LightType;
use structs::Sphere;

mod color;

use color::Color;

mod vector;

use vector::Vector;
use vector::dot;

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
const LIGHTS: [Light; 3] = [
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
const SPHERES: [Sphere; 4] = [
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
    },
];

fn canvas_to_viewport(x: f32, y: f32) -> Vector {
    // I am not sure why I need to invert the y axis - maybe something to do with .ppm
    // NOTE: Consider removing inversion later if another output format corrects it
    Vector {
        x: (x * VIEWPORT_SIZE / CANVAS.width),
        y: -(y * VIEWPORT_SIZE / CANVAS.height),
        z: (PROJECTION_PLANE_Z),
    }
}

fn intersect_ray_sphere(origin: Vector, direction: Vector, sphere: Sphere) -> Vec<f32> {
    let r = sphere.radius;
    let co = origin - sphere.center;

    let a = dot(direction, direction);
    let b = 2.0 * dot(co, direction);
    let c = dot(co, co) - r * r;

    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        vec![f32::INFINITY, f32::INFINITY]
    } else {
        let t1 = (-b + f32::sqrt(discriminant)) / (2.0 * a);
        let t2 = (-b - f32::sqrt(discriminant)) / (2.0 * a);
        vec![t1, t2]
    }
}

fn compute_lighting(point: Vector, normal: Vector) -> f32 {
    let mut intensity = 0.0;
    let mut light_vector;

    for light in LIGHTS.iter() {
        if light.light_type == LightType::Ambient {
            intensity += light.intensity;
        } else {
            let light_position = light.light_position.unwrap();

            if light.light_type == LightType::Point {
                light_vector = light_position - point;
            } else {
                light_vector = light_position;
            }

            let n_dot_1 = dot(normal, light_vector);
            if n_dot_1 > 0.0 {
                intensity += light.intensity * n_dot_1 / (normal.length() * light_vector.length());
            }
        }
    }
    intensity
}

fn trace_ray(origin: Vector, direction: Vector, t_min: f32, t_max: f32) -> Option<Color> {
    let mut closest_t = f32::INFINITY;
    let mut closest_sphere = None;

    for sphere in SPHERES.iter() {
        let ts = intersect_ray_sphere(origin, direction, *sphere);
        if (t_min < ts[0]) && (ts[0] < t_max) && (ts[0] < closest_t) {
            closest_t = ts[0];
            closest_sphere = Some(sphere);
        }
        // NOTE: this was an 'else if' and that messed with lighting and put some spheres in front of others
        if (t_min < ts[1]) && (ts[1] < t_max) && (ts[1] < closest_t) {
            closest_t = ts[1];
            closest_sphere = Some(sphere);
        }
    }

    closest_sphere.map(|i| {
        let point = origin + closest_t * direction;
        let mut normal = point - i.center;
        normal = normal / normal.length();
        i.color * compute_lighting(point, normal)
    })
}

fn put_pixel(x: f32, y: f32, color: Option<Color>, file: &mut File) {
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
        // .ppm format expects integers
        Some(i) => writeln!(file, "{}, {}, {}", i.r as i32, i.g as i32, i.b as i32)
            .expect("Error writing color to file."),
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
            let color = trace_ray(CAMERA_POSITION, direction, 1.0, f32::INFINITY);

            put_pixel(x as f32, y as f32, color, &mut file);
        }
    }
}
