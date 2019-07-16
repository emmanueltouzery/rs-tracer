// based on the "Ray Tracing in a Weekend" book

#[macro_use] extern crate impl_ops;

mod v3color;
mod shapes;
mod camera;
mod material;
use {v3color::*, shapes::*, camera::*, material::*};

use std::cmp;
use rand::{prelude as random, Rng};

static WIDTH: i32 = 200;
static HEIGHT: i32 = 100;
static ANTIALIAS_SAMPLES: i32 = 100;

static BLACK_V: V3 = V3 { x: 0.0, y: 0.0, z: 0.0};

fn print_color(col: Color) -> String {
    let to_component = |c| (255.99 * f32::sqrt(c)) as i32;
    format!("{} {} {}", 
        to_component(col.r),
        to_component(col.g),
        to_component(col.b))
}

fn closest_hit<'a>(objects: &'a [Box<Shape>], ray: &Ray, t_range: &std::ops::Range<f32>) -> Option<HitRecord<'a>> {
    objects
        .iter()
        .flat_map(|o| o.hit(ray, t_range))
        // https://www.reddit.com/r/rust/comments/29kia3/no_ord_for_f32/cilrzik/
        .min_by(|o1, o2| o1.t.partial_cmp(&o2.t).unwrap_or(cmp::Ordering::Equal))
}

fn color_for_ray(objects: &[Box<Shape>], ray: &Ray, depth: i32) -> Color {
    _color_for_ray(objects, ray, depth).to_color()
}

fn _color_for_ray(objects: &[Box<Shape>], ray: &Ray, depth: i32) -> V3 {
    if depth >= 50 {
        return BLACK_V;
    }
    match closest_hit(objects, ray, &(0.001..std::f32::MAX)) {
        Some(r) => {
            r.material.scatter(ray, &r)
                .map_or_else(|| BLACK_V, |scatter_info| {
                    scatter_info.attenuation
                        * _color_for_ray(objects, &scatter_info.scattered, depth+1)
                })
        }
        None => {
            let unit_direction = ray.direction.unit();
            let t = 0.5 * (unit_direction.y + 1.0);
            ((1.0-t) * V3 { x: 1.0, y: 1.0, z: 1.0 }
                + t*V3 { x: 0.5, y: 0.7, z: 1.0 })
        }
    }
 }

fn main() {
    let objects: Vec<Box<Shape>> = vec![
        Box::new(Sphere {
            center: V3 { x: 0.0, y: 0.0, z: -1.0 },
            radius: 0.5,
            material: Box::new(Lambertian { albedo: Color { r: 0.1, g: 0.2, b: 0.5} })
        }),
        Box::new(Sphere {
            center: V3 { x: 0.0, y: -100.5, z: -1.0},
            radius: 100.0,
            material: Box::new(Lambertian { albedo: Color { r: 0.8, g: 0.8, b: 0.0}})
        }),
        Box::new(Sphere {
            center: V3 { x: 1.0, y: 0.0, z: -1.0 },
            radius: 0.5,
            material: Box::new(Metal { 
                albedo: Color { r: 0.8, g: 0.6, b: 0.2}, 
                fuzz: 0.0 
            })
        }),
        Box::new(Sphere {
            center: V3 { x: -1.0, y: 0.0, z: -1.0},
            radius: 0.5,
            material: Box::new(Dielectric { ref_idx: 1.5 })
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
                let cur_col = color_for_ray(&objects, &ray, 0);
                col_vec.x += cur_col.r;
                col_vec.y += cur_col.g;
                col_vec.z += cur_col.b;
            }
            let col = (col_vec / ANTIALIAS_SAMPLES as f32).to_color();
            println!("{}", print_color(col));
        }
    }
}
