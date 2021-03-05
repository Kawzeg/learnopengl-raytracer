use crate::renderer::Renderer;

#[derive(Clone, Copy, Debug)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn mag(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn norm(&self) -> Vec3 {
        *self / self.mag()
    }

    fn theta(&self, other: &Vec3) -> f64 {
        (self.dot(other.norm()) / self.mag()).acos()
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[derive(Debug)]
struct Ray {
    p: Vec3,
    q: Vec3,
}
struct Sphere {
    pos: Vec3,
    r: f64,
}

impl Sphere {
    fn distance(r: &Ray, p: Vec3) -> f64 {
        (r.q - r.p).cross(r.p - p).mag() / (r.q - r.p).mag()
    }
}

impl Renderable for Sphere {
    fn intersects(&self, r: &Ray) -> bool {
        if (r.q - r.p).theta(&(self.pos - r.p)) > 90.0_f64.to_radians() {
            return false;
        }
        let d = Sphere::distance(r, self.pos);
        d < self.r
    }
}

trait Renderable {
    fn intersects(&self, l: &Ray) -> bool;
}

pub struct Raytracer {
    scene: Vec<Box<dyn Renderable>>,
    pos: Vec3,
    dir: Vec3,
    near_plane: f64,
    fov: f64,
}

impl Raytracer {
    pub fn new() -> Raytracer {
        Raytracer {
            scene: vec![Box::new(Sphere {
                pos: Vec3 {
                    x: 0.,
                    y: 0.,
                    z: 100.,
                },
                r: 30.,
            })],
            pos: Vec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            dir: Vec3 {
                x: 0.,
                y: 0.,
                z: 1.,
            },
            near_plane: 1.,
            fov: 60.,
        }
    }

    fn frustum(&self) -> (f64, f64, f64, f64) {
        let x0 = 2. * self.near_plane * self.fov.tan();
        let x1 = -x0;
        let ratio = (WIDTH as f64) / (HEIGHT as f64);
        let y0 = x0 / ratio;
        let y1 = -y0;
        let dx = (x1 - x0) / WIDTH as f64;
        let dy = (y1 - y0) / HEIGHT as f64;
        (x0, y0, dx, dy)
    }
}

const WIDTH: u16 = 2000;
const HEIGHT: u16 = 1500;

impl Renderer for Raytracer {
    fn render(&mut self, _t: f64) -> (Vec<u8>, u16, u16) {
        let (x0, y0, dx, dy) = self.frustum();

        let mut pixels = vec![0x00; 4 * WIDTH as usize * HEIGHT as usize];

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let ray = Ray {
                    p: self.pos,
                    q: Vec3 {
                        x: x0 + (dx * x as f64),
                        y: y0 + (dy * y as f64),
                        z: 1.,
                    },
                };
                for obj in &self.scene {
                    if obj.intersects(&ray) {
                        let i = (((y as usize * WIDTH as usize) + x as usize) * 4) as usize;
                        pixels[i] = 0x18;
                        pixels[i + 1] = 0x39;
                        pixels[i + 2] = 0x3E;
                        pixels[i + 3] = 0xFF;
                        continue;
                    }
                }
            }
        }

        (pixels, WIDTH, HEIGHT) // FIXME remove
    }
}