// based on the "Ray Tracing in a Weekend" book

#[macro_use] extern crate impl_ops;

mod v3color;
use v3color::*;

static WIDTH: i32 = 200;
static HEIGHT: i32 = 100;

struct Ray {
    origin: V3,
    direction: V3
}

fn print_color(col: Color) -> String {
    let to_component = |c| (255.99 * c) as i32;
    format!("{} {} {}", 
        to_component(col.r),
        to_component(col.g),
        to_component(col.b))
}

fn hit_sphere(center: V3, radius: f32, ray: &Ray) -> bool {
    let oc = ray.origin - center;
    let a = dot(&ray.direction, &ray.direction);
    let b = 2.0 * dot(&oc, &ray.direction);
    let c = dot(&oc, &oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    discriminant > 0.0
}

fn color_for_ray(ray: &Ray) -> Color {
    if hit_sphere(V3 {x: 0.0, y: 0.0, z: -1.0}, 0.5, ray) {
        return Color { r: 1.0, g: 0.0, b: 0.0};
    }
    let unit_direction = unit_vector(&ray.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    v3_to_color(&((1.0-t) * V3 { x: 1.0, y: 1.0, z: 1.0 }
        + t*V3 { x: 0.5, y: 0.7, z: 1.0 }))
}

fn main() {
    println!("P3\n{} {}\n255", WIDTH, HEIGHT);

    let lower_left_corner = V3 { x: -2.0, y: -1.0, z: -1.0 };
    let origin = V3 { x: 0.0, y: 0.0, z: 0.0 };
    let horizontal = V3 { x: 4.0, y: 0.0, z: 0.0 };
    let vertical = V3 { x: 0.0, y: 2.0, z: 0.0 };

    for j in (0..HEIGHT).rev() {
        for i in 0..WIDTH {
            let u = i as f32 / WIDTH as f32;
            let v = j as f32 / HEIGHT as f32;
            let ray = Ray {
                origin,
                direction: lower_left_corner + u*horizontal + v*vertical
            };
            println!("{}", print_color(color_for_ray(&ray)));
        }
    }
}
