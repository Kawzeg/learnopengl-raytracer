use std::option;

use super::{Ray, Rgb, renderable::Renderable, vec3::Vec3};

pub struct Plane {
    /// A point on the plane
    pub pos: Vec3,
    /// Normal Vector of the plane
    pub n: Vec3,
}

impl Plane {}

impl Renderable for Plane {
    fn intersects(&self, r: &Ray) -> (bool, Ray, Rgb) {
        // https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection
        let u = (r.q - r.p).norm(); // Unit direction vector

        let udotn = u.dot(self.n);

        let EPSILON = 0.00005;
        if udotn.abs() - EPSILON < 0.  { // Parallel case
            return (false, Ray::NULL, Rgb::BLACK);
        }

        let k = (self.pos - r.p).dot(self.n) / udotn;
        
        let intersection = r.p + (u * k);
        let reflection: Vec3 = u - (self.n * (udotn * 2.));
        (
            true,
            Ray {
                p: intersection,
                q: reflection,
            },
            Rgb::BLACK,
        )
    }
}
