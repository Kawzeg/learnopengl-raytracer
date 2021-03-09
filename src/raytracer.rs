use std::f64::consts::PI;

mod plane;
mod renderable;
mod sphere;
mod vec3;

use crate::renderer::Renderer;
use crate::util::normalize;
use plane::Plane;
use renderable::Renderable;
use sphere::Sphere;
use vec3::Vec3;

struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Rgba {
    const BLACK: Rgba = Rgba {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };
}

impl From<Rgb> for Rgba {
    fn from(other: Rgb) -> Self {
        Rgba {
            r: other.r,
            g: other.g,
            b: other.b,
            a: 0xFF,
        }
    }
}

#[derive(Clone, Copy)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Rgb {
    const BLACK: Rgb = Rgb { r: 0, g: 0, b: 0 };
}

#[derive(Debug)]
struct Ray {
    p: Vec3,
    q: Vec3,
}

impl Ray {
    const NULL: Ray = Ray {
        p: Vec3::NULL,
        q: Vec3::NULL,
    };
}

trait Light {
    /// TODO How?
    fn color(&self, l: &Vec3) -> Rgba;
}

struct Scene {
    objects: Vec<Box<dyn Renderable>>,
    lights: Vec<Box<dyn Light>>,
    sunlight: Vec3,
}

pub struct Raytracer {
    scene: Scene,
    pos: Vec3,
    dir: Vec3,
    near_plane: f64,
    fov: f64,
}

impl Raytracer {
    pub fn new() -> Raytracer {
        Raytracer {
            scene: Scene {
                objects: vec![
                    Box::new(Sphere {
                        pos: SPHERE_POS,
                        r: 30.,
                        color: Rgb {
                            r: 0x18,
                            g: 0x39,
                            b: 0x3E,
                        },
                    }), /*
                        Box::new(Plane {
                            pos: Vec3 {
                                x: 0.,
                                y: 0.,
                                z: 0.,
                            },
                            n: Vec3 {
                                x: 0.,
                                y: 1.,
                                z: 0.,
                            },
                        }),*/
                ],
                lights: vec![],
                sunlight: Vec3 {
                    x: 5.,
                    y: -3.,
                    z: 1.,
                },
            },
            pos: START_POS,
            dir: (SPHERE_POS - START_POS).norm(),
            near_plane: 1.,
            fov: 60.,
        }
    }

    /// Returns three Vec3s: bottom left corner, dx, and dy
    fn frustum(&self) -> (Vec3, Vec3, Vec3) {
        let left = self.dir.cross(Vec3::UP);
        let down = self.dir.cross(left);

        // Calculate top left corner
        let center = self.near_plane * self.dir + self.pos;
        // x and y unit distance
        let x0 = 2. * self.near_plane * self.fov.tan();
        let ratio = (WIDTH as f64) / (HEIGHT as f64);
        let y0 = x0 / ratio;
        let bottomleft: Vec3 = center + left * x0 + down * y0;

        let width = x0 * -2.;
        let height = y0 * -2.;

        let dx = (width / WIDTH as f64) * left;
        let dy = (height / HEIGHT as f64) * down;

        (bottomleft, dx, dy)
    }
}

const WIDTH: u16 = 400;
const HEIGHT: u16 = 300;
const SPHERE_POS: Vec3 = Vec3 {
    x: 0.,
    y: 0.,
    z: 100.,
};
const START_POS: Vec3 = Vec3 {
    x: 0.,
    y: 0.,
    z: 0.,
};

fn intersect(ray: &Ray, scene: &Scene, depth: u32) -> Option<Rgb> {
    for obj in &scene.objects {
        let (intersects, reflection, color) = obj.intersects(&ray);
        if intersects {
            let sun_angle = scene.sunlight.theta(&reflection.q);
            let lightness: f64;
            if sun_angle < PI / 2. {
                // More than 90°
                lightness = 0.;
            } else {
                lightness = normalize(sun_angle - (PI / 2.), PI / 2.);
            }
            let gray: u8 = (lightness * 255.).round() as u8;

            /*
            pixels[i] = (lightness * r).round() as u8;
            pixels[i + 1] = (lightness * g).round() as u8;
            pixels[i + 2] = (lightness * b).round() as u8;
            */
            /*let color = Rgba {
                r: gray,
                g: gray,
                b: gray,
                a: 0xFF,
            };
            */
            if depth > 0 {
                let reflected = intersect(&reflection, scene, depth - 1);
                match reflected {
                    other @ Some(_) => return other,
                    None => return Some(color),
                }
            }
            return Some(color);
        }
    }
    None
}

impl Renderer for Raytracer {
    fn render(&mut self, _t: f64) -> (Vec<u8>, u16, u16) {
        let (bottomleft, dx, dy) = self.frustum();

        let mut pixels = vec![0x00; 4 * WIDTH as usize * HEIGHT as usize];

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let ray = Ray {
                    p: self.pos,
                    q: bottomleft + dx * (x as f64) + dy * (y as f64),
                };
                let color = match intersect(&ray, &self.scene, 0) {
                    Some(x) => Rgba::from(x),
                    None => Rgba::BLACK,
                };
                let i = (((y as usize * WIDTH as usize) + x as usize) * 4) as usize;
                pixels[i] = color.r;
                pixels[i + 1] = color.g;
                pixels[i + 2] = color.b;
                pixels[i + 3] = color.a;
            }
        }

        let left = self.dir.cross(Vec3::UP).norm();
        self.pos = self.pos - left;
        self.dir = (SPHERE_POS - self.pos).norm();
        (pixels, WIDTH, HEIGHT)
    }
}
