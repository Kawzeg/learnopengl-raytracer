use super::{hit::Hit, renderable::Renderable, vec3::Vec3, Ray, Rgb};

pub struct Plane {
    /// A point on the plane
    pub pos: Vec3,
    /// Normal Vector of the plane
    pub n: Vec3,
    pub(super) color: Rgb,
    pub reflectivity: f64,
}

impl Plane {}

impl Renderable for Plane {
    fn intersects(&self, r: &Ray) -> Option<Hit> {
        // https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection
        let u = (r.q - r.p).norm(); // Unit direction vector

        let udotn = u.dot(self.n);

        let epsilon = 0.00005;
        if udotn.abs() - epsilon < 0. {
            // Parallel case
            return None;
        }

        let k = (self.pos - r.p).dot(self.n) / udotn;

        if k < 1. {
            // Behind the start of the ray
            return None;
        }

        let intersection = r.p + (u * k);
        let reflection: Vec3 = u - (self.n * (udotn * 2.));
        Some(Hit {
            reflection: Ray {
                p: intersection,
                q: intersection + reflection,
            },
            color: self.color,
            reflectivity: self.reflectivity,
        })
    }
}