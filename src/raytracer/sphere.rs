use super::{hit::Hit, renderable::Renderable, vec3::Vec3, Ray, Rgb};

pub struct Sphere {
    pub pos: Vec3,
    pub r: f64,
    pub(super) color: Rgb,
    pub reflectivity: f64,
}

impl Sphere {
    fn distance(r: &Ray, p: Vec3) -> f64 {
        (r.q - r.p).cross(r.p - p).mag() / (r.q - r.p).mag()
    }
}

impl Renderable for Sphere {
    fn intersects(&self, r: &Ray) -> Option<Hit> {
        // https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection
        let u = (r.q - r.p).norm(); // Unit direction vector
        let o = r.p;
        let c = self.pos;

        let udotoc = u.dot(o - c);
        let ocmag = (o - c).mag();
        let nabla = (udotoc * udotoc) - ((ocmag * ocmag) - (self.r * self.r));
        if nabla <= 0. {
            return None; // No solution / Tangential
        }
        let k1 = -(udotoc) + nabla.sqrt();
        let k2 = -(udotoc) - nabla.sqrt();
        let k = k1.min(k2);

        if k < 1. {
            return None; // Behind the start of the ray
        }

        let intersection: Vec3 = r.p + (u * k);
        // Reflection
        let n = (intersection - self.pos).norm();
        let reflection: Vec3 = u - (((u * 2.).dot(n)) / (n.mag() * n.mag())) * n;
        // let reflection: Vec3 = u - (n * (u.dot(n) * 2.));
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
