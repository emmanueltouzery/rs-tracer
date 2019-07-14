// based on the "Ray Tracing in a Weekend" book

#[macro_use] extern crate impl_ops;

mod v3color;
mod shapes;
use {v3color::*, shapes::*};
use std::cmp;

static WIDTH: i32 = 200;
static HEIGHT: i32 = 100;

fn print_color(col: Color) -> String {
    let to_component = |c| (255.99 * c) as i32;
    format!("{} {} {}", 
        to_component(col.r),
        to_component(col.g),
        to_component(col.b))
}

fn closest_hit(objects: &[Box<Shape>], ray: &Ray, t_range: &std::ops::Range<f32>) -> Option<HitRecord> {
    objects
        .iter()
        .flat_map(|o| o.hit(ray, t_range))
        // https://www.reddit.com/r/rust/comments/29kia3/no_ord_for_f32/cilrzik/
        .min_by(|o1, o2| o1.t.partial_cmp(&o2.t).unwrap_or(cmp::Ordering::Equal))
}

fn color_for_ray(objects: &[Box<Shape>], ray: &Ray) -> Color {
    match closest_hit(objects, ray, &(0.0..std::f32::MAX)) {
        Some(r) => {
            (0.5*(r.normal + V3 { x: 1.0, y: 1.0, z: 1.0})).to_color()
        }
        None => {
            let unit_direction = ray.direction.unit();
            let t = 0.5 * (unit_direction.y + 1.0);
            ((1.0-t) * V3 { x: 1.0, y: 1.0, z: 1.0 }
                + t*V3 { x: 0.5, y: 0.7, z: 1.0 }).to_color()
        }
    }
 }

fn main() {
    let objects: Vec<Box<Shape>> = vec![
        Box::new(Sphere {
            center: V3 { x: 0.0, y: 0.0, z: -1.0 },
            radius: 0.5
        }),
        Box::new(Sphere {
            center: V3 { x: 0.0, y: -100.5, z: -1.0},
            radius: 100.0
        })
    ];

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
            println!("{}", print_color(color_for_ray(&objects, &ray)));
        }
    }
}
