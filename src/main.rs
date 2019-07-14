// based on the "Ray Tracing in a Weekend" book

#[macro_use] extern crate impl_ops;

mod v3color;
mod shapes;
mod camera;
use {v3color::*, shapes::*, camera::*};

use std::cmp;
use rand::{prelude as random, Rng};

static WIDTH: i32 = 200;
static HEIGHT: i32 = 100;
static ANTIALIAS_SAMPLES: i32 = 100;

fn print_color(col: Color) -> String {
    let to_component = |c| (255.99 * f32::sqrt(c)) as i32;
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

fn random_in_unit_sphere() -> V3 {
    let mut rng = random::thread_rng();
    let mut p;
    let unit = V3 { x: 1.0, y: 1.0, z: 1.0};
    loop {
        p = 2.0 * V3 { 
            x: rng.gen::<f32>(),
            y: rng.gen::<f32>(),
            z: rng.gen::<f32>()
        } - unit;
        if p.squared_length() < 1.0 { break p; }
    }
}

fn color_for_ray(objects: &[Box<Shape>], ray: &Ray) -> Color {
    match closest_hit(objects, ray, &(0.001..std::f32::MAX)) {
        Some(r) => {
            let target = r.p + r.normal + random_in_unit_sphere();
            let col = color_for_ray(objects,
                &Ray { origin: r.p, direction: target - r.p });
            Color {
                r: col.r/2.0,
                g: col.g/2.0,
                b: col.b/2.0
            }
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

    let camera = Camera::default();

    let mut rng = random::thread_rng();
    for j in (0..HEIGHT).rev() {
        for i in 0..WIDTH {
            let mut col_vec = V3 { x: 0.0, y: 0.0, z: 0.0 };
            for _ in 0..ANTIALIAS_SAMPLES {
                let u = (i as f32 + rng.gen::<f32>()) / WIDTH as f32;
                let v = (j as f32 + rng.gen::<f32>()) / HEIGHT as f32;
                let ray = camera.get_ray(u, v);
                let cur_col = color_for_ray(&objects, &ray);
                col_vec.x += cur_col.r;
                col_vec.y += cur_col.g;
                col_vec.z += cur_col.b;
            }
            let col = (col_vec / ANTIALIAS_SAMPLES as f32).to_color();
            println!("{}", print_color(col));
        }
    }
}
