// based on the "Ray Tracing in a Weekend" book

#[macro_use] extern crate impl_ops;
extern crate rayon;

mod v3color;
mod shapes;
mod camera;
mod material;
use {v3color::*, shapes::*, camera::*, material::*};

use std::cmp;
use rand::{prelude as random, Rng};
use rayon::prelude::*;

static WIDTH: i32 = 800;
static HEIGHT: i32 = 400;
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

fn scene() -> Vec<Box<Shape>> {
    let mut rng = random::thread_rng();
    let mut objects: Vec<Box<Shape>> = vec![
        Box::new(Sphere {
            center: V3 { x: 0.0, y: -1000.0, z: 0.0},
            radius: 1000.0,
            material: Box::new(Lambertian { albedo: Color { r: 0.5, g: 0.5, b: 0.5}})
        })
    ];
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f32>();
            let center = V3 {
                x: a as f32 + 0.9*rng.gen::<f32>(),
                y: 0.2,
                z: b as f32 + 0.9*rng.gen::<f32>()
            };
            if (center - V3 {x: 4.0, y: 0.2, z: 0.0}).length() > 0.9 {
                if choose_mat < 0.7 { // diffuse
                    objects.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(Lambertian {
                            albedo: Color {
                                r: rng.gen::<f32>()*rng.gen::<f32>(),
                                g: rng.gen::<f32>()*rng.gen::<f32>(),
                                b: rng.gen::<f32>()*rng.gen::<f32>()
                            }
                        })
                    }));
                } else if choose_mat < 0.85 { // metal
                    objects.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(Metal {
                            albedo: Color {
                                r: 0.5 * (1.0+rng.gen::<f32>()),
                                g: 0.5 * (1.0+rng.gen::<f32>()),
                                b: 0.5 * (1.0+rng.gen::<f32>()),
                            },
                            fuzz: 0.5*rng.gen::<f32>()
                        })
                    }))
                } else { // glass
                    objects.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(Dielectric {
                            ref_idx: 1.5
                        })
                    }))
                }
            }
        }
    }
    let extra: Vec<Box<Shape>> = vec![
         Box::new(Sphere {
            center: V3 { x: 0.0, y: 1.0, z: 0.0 },
            radius: 1.0,
            material: Box::new(Dielectric { ref_idx: 1.5 })
        }),
        Box::new(Sphere {
            center: V3 { x: -4.0, y: 1.0, z: 0.0},
            radius: 1.0,
            material: Box::new(Lambertian { albedo: Color { r: 0.4, g: 0.2, b: 0.1}})
        }),
        Box::new(Sphere {
            center: V3 { x: 4.0, y: 1.0, z: 0.0 },
            radius: 1.0,
            material: Box::new(Metal { 
                albedo: Color { r: 0.7, g: 0.6, b: 0.5}, 
                fuzz: 0.0
            })
        })
    ];
    objects.extend(extra);
    objects
}

fn main() {
    println!("P3\n{} {}\n255", WIDTH, HEIGHT);

    let objects = scene();

    let aperture = 0.05;
    let look_from = V3 {x: 10.0, y: 1.8, z: 2.6};
    let look_at = V3 {x: 0.0, y: 0.5, z: 0.0};
    let dist_to_focus = (look_from - V3 {x: 4.0, y: 1.0, z: 0.0}).length();
    let camera = Camera::new(
        &look_from,
        &look_at,
        &V3 {x: 0.0, y: 1.0, z: 0.0},
        20.0, WIDTH as f32 / HEIGHT as f32,
        aperture, dist_to_focus);

    let row_cols = (0..HEIGHT).rev().collect::<Vec<_>>().par_iter().map(|&j| {
        let mut rng = random::thread_rng();
        (0..WIDTH).map(|i| {
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
            (col_vec / ANTIALIAS_SAMPLES as f32).to_color()
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    for row_col in row_cols {
        for pixel in row_col {
            println!("{}", print_color(pixel));
        }
    }
}
