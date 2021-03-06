// based on the "Ray Tracing in a Weekend" book

#[macro_use] extern crate impl_ops;
extern crate rayon;

mod v3color;
mod shapes;
mod camera;
mod material;
mod bvh;
mod texture;
mod perlin;
use {
    v3color::*, shapes::*, camera::*, 
    material::*, bvh::*, texture::*, perlin::*
    };

use std::env;
use std::sync::atomic::{AtomicUsize, Ordering};
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
        .min_by(|o1, o2| f32_cmp(o1.t, o2.t))
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
                    scatter_info.attenuation.to_v3()
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

fn two_spheres_scene() -> Vec<Box<Shape>> {
    let checker = || Box::new(SphericalCheckerTexture {
        even: Box::new(ConstantTexture { color: Color { r: 0.2, g: 0.3, b: 0.1 } }),
        odd: Box::new(ConstantTexture { color: Color { r: 0.9, g: 0.9, b: 0.9 } }),
    });
    vec![
        Box::new(Sphere {
            center: V3 { x: 0.0, y: -10.0, z: 0.0},
            radius: 10.0,
            material: Box::new(Lambertian {
                albedo: checker()
            })
        }),
        Box::new(Sphere {
            center: V3 { x: 0.0, y: 10.0, z: 0.0},
            radius: 10.0,
            material: Box::new(Lambertian {
                albedo: checker()
            })
        })
    ]
}

fn noise_two_spheres_scene() -> Vec<Box<Shape>> {
    let noise_t = || Box::new(NoiseTexture::new());
    vec![
        Box::new(Sphere {
            center: V3 { x: 0.0, y: -1000.0, z: 0.0},
            radius: 1000.0,
            material: Box::new(Lambertian {
                albedo: noise_t()
            })
        }),
        Box::new(Sphere {
            center: V3 { x: 0.0, y: 2.0, z: 0.0},
            radius: 2.0,
            material: Box::new(Lambertian {
                albedo: noise_t()
            })
        })
    ]
}

fn scene() -> Vec<Box<Shape>> {
    let mut rng = random::thread_rng();
    let checker = Box::new(CheckerTexture {
        even: Box::new(ConstantTexture { color: Color { r: 0.2, g: 0.3, b: 0.1 } }),
        odd: Box::new(ConstantTexture { color: Color { r: 0.9, g: 0.9, b: 0.9 } }),
    });
    let mut objects: Vec<Box<Shape>> = vec![
        Box::new(Sphere {
            center: V3 { x: 0.0, y: -1000.0, z: 0.0},
            radius: 1000.0,
            material: Box::new(Lambertian { albedo: checker })
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
                    objects.push(Box::new(MovingSphere {
                        center0: center,
                        center1: center+ V3 {x: 0.0, y: 0.5*rng.gen::<f32>(), z: 0.0},
                        time0: 0.0,
                        time1: 1.0,
                        radius: 0.2,
                        material: Box::new(Lambertian {
                            albedo: Box::new(ConstantTexture {
                                color: Color {
                                    r: rng.gen::<f32>()*rng.gen::<f32>(),
                                    g: rng.gen::<f32>()*rng.gen::<f32>(),
                                    b: rng.gen::<f32>()*rng.gen::<f32>()
                                }
                            })
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
            material: Box::new(Lambertian { 
                albedo: Box::new(ConstantTexture {
                        color: {Color { r: 0.4, g: 0.2, b: 0.1}}
                    })
            })
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
    let args: Vec<String> = env::args().collect();

    println!("P3\n{} {}\n255", WIDTH, HEIGHT);

    let objects = vec![BvhNode::compute_shapes_bvh(
        match args[1].as_ref() {
            "--two-spheres" => two_spheres_scene(),
            "--noise" => noise_two_spheres_scene(),
            _ => scene()
        },
        &(0.001..std::f32::MAX))];
    // let objects = scene();

    let look_from = V3 {x: 10.0, y: 1.8, z: 2.6};
    let look_at = V3 {x: 0.0, y: 0.5, z: 0.0};
    let camera = Camera::new(CameraParams {
        look_from: &look_from,
        look_at: &look_at,
        vup: &V3 {x: 0.0, y: 1.0, z: 0.0},
        focus_dist: (look_from - V3 {x: 4.0, y: 1.0, z: 0.0}).length(),
        aperture: 0.05,
        aspect: WIDTH as f32 / HEIGHT as f32,
        vert_fov_deg: 20.0,
        time1: 0.0,
        time2: 1.0
    });

    let rendered_rows = AtomicUsize::new(0);

    eprint!("Rendered {:3}%", 0);
    // use par_iter to render rows in a multithread manner using the rayon library.
    let row_cols = (0..HEIGHT).rev().collect::<Vec<_>>().par_iter().map(|&j| {
        let mut rng = random::thread_rng();
        let row_cols = (0..WIDTH).map(|i| {
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
        }).collect::<Vec<_>>();
        let rendered = rendered_rows.fetch_add(1, Ordering::SeqCst)+1;
        if rendered % 10 == 0 {
            eprint!("\rRendered {:3}%", (rendered as i32) * 100 / HEIGHT);
        }
        row_cols
    }).collect::<Vec<_>>();
    for row_col in row_cols {
        for pixel in row_col {
            println!("{}", print_color(pixel));
        }
    }
    eprint!("\r");
}
