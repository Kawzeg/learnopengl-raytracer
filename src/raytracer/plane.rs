use super::{hit::Hit, renderable::Renderable, vec3::Vec3, Ray, Rgb};

pub struct Plane {
    /// A point on the plane
    pub pos: Vec3,
    /// Normal Vector of the plane
    pub n: Vec3,
    pub(super) color: Rgb,
    pub reflectivity: f64,
    pub checker: bool,
}

impl Plane {}

impl Renderable for Plane {
    fn intersects(&self, r: &Ray, _t: f64) -> Option<Hit> {
        // https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection
        let u = (r.q - r.p).norm(); // Unit direction vector

        if u.theta(&self.n) < 90.0_f64.to_radians() {
            // Ray comes from behind plane
            return None;
        }

        let udotn = u.dot(self.n);

        let epsilon = 0.00005;
        if udotn.abs() < epsilon {
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

        let color;
        let reflectivity;
        if self.checker {
            let black = ((intersection.x / 100.).floor() as i64
                + (intersection.z / 100.).floor() as i64)
                % 2
                == 0;
            color = if black { Rgb::BLACK } else { Rgb::WHITE };
            reflectivity = if black { self.reflectivity } else { 0.3 };
        } else {
            color = self.color;
            reflectivity = self.reflectivity;
        }

        Some(Hit {
            reflection: Ray {
                p: intersection,
                q: intersection + reflection,
            },
            color,
            reflectivity,
        })
    }
}
