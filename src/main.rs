// my mods
mod scene;
use scene::BACKGROUND_COLOR;
use scene::CAMERA_POSITION;
use scene::CANVAS;
use scene::LIGHTS;
use scene::SPHERES;

mod structs;

use structs::Canvas;
use structs::LightType;
use structs::Sphere;

mod color;

use color::Color;

mod vector;

use vector::Vector;
use vector::dot;

// globals

const EPSILON: f32 = 0.001;
const PROJECTION_PLANE_Z: f32 = 1.0; // distance from camera to projection plane
const VIEWPORT_SIZE: f32 = 1.0;

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

fn compute_lighting(point: Vector, normal: Vector, view_direction: Vector, specular: f32) -> f32 {
    let mut intensity = 0.0;
    let mut light_vector;
    let mut t_max;

    for light in LIGHTS.iter() {
        if light.light_type == LightType::Ambient {
            intensity += light.intensity;
        } else {
            let light_position = light.light_position.unwrap();

            if light.light_type == LightType::Point {
                light_vector = light_position - point;
                t_max = 1.0;
            } else {
                light_vector = light_position;
                t_max = f32::INFINITY;
            }

            // shadow
            let (shadow_sphere, _) = closest_intersection(point, light_vector, EPSILON, t_max);

            match shadow_sphere {
                None => {
                    // diffuse
                    let n_dot_1 = dot(normal, light_vector);
                    if n_dot_1 > 0.0 {
                        intensity +=
                            light.intensity * n_dot_1 / (normal.length() * light_vector.length());
                    }

                    // specular
                    if specular != -1.0 {
                        let reflection_vector =
                            2.0 * normal * dot(normal, light_vector) - light_vector;
                        let r_dot_v = dot(reflection_vector, view_direction);
                        if r_dot_v > 0.0 {
                            intensity += light.intensity
                                * f32::powf(
                                    r_dot_v
                                        / (reflection_vector.length() * view_direction.length()),
                                    specular,
                                );
                        }
                    }
                }
                Some(_) => continue,
            }
        }
    }
    intensity
}

fn closest_intersection(
    origin: Vector,
    direction: Vector,
    t_min: f32,
    t_max: f32,
) -> (Option<Sphere>, f32) {
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
    (closest_sphere.copied(), closest_t)
}

fn reflect_ray(ray: Vector, normal: Vector) -> Vector {
    2.0 * normal * dot(normal, ray) - ray
}

fn trace_ray(
    origin: Vector,
    direction: Vector,
    t_min: f32,
    t_max: f32,
    recursion_depth: i32,
) -> Color {
    let (closest_sphere, closest_t) = closest_intersection(origin, direction, t_min, t_max);

    match closest_sphere {
        None => BACKGROUND_COLOR,
        Some(i) => {
            let point = origin + closest_t * direction;
            let mut normal = point - i.center;
            normal = normal / normal.length();
            let local_color = i.color * compute_lighting(point, normal, -direction, i.specular);

            let reflectivity = i.reflective;
            if recursion_depth <= 0 || reflectivity <= 0.0 {
                return local_color;
            }

            let reflected_ray = reflect_ray(-direction, normal);
            let reflected_color = trace_ray(
                point,
                reflected_ray,
                EPSILON * 100.0,
                f32::INFINITY,
                recursion_depth - 1,
            );

            local_color * (1.0 - reflectivity) + reflected_color * reflectivity
        }
    }
}

fn main() {
    let mut imgbuf: image::ImageBuffer<image::Rgb<u8>, _> =
        image::ImageBuffer::new(CANVAS.width as u32, CANVAS.height as u32);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let view_x = x as f32 - CANVAS.width / 2.0;
        let view_y = y as f32 - CANVAS.width / 2.0;
        let direction = canvas_to_viewport(view_x, view_y);
        let color = trace_ray(CAMERA_POSITION, direction, 1.0, f32::INFINITY, 3);

        *pixel = image::Rgb([color.r as u8, color.g as u8, color.b as u8]);
    }

    imgbuf.save("image.png").expect("Image failed to save.");
}
