use super::{hit::Hit, renderable::Renderable, vec3::Vec3, Ray, Rgb};

pub struct MovingSphere<T>
where
    T: Fn(f64) -> Vec3,
{
    pub pos: T,
    pub r: f64,
    pub(super) color: Rgb,
    pub reflectivity: f64,
}

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

    fn intersects(
        pos: Vec3,
        radius: f64,
        ray: &Ray,
        color: Rgb,
        reflectivity: f64,
    ) -> Option<Hit> {
        // https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection
        let u = (ray.q - ray.p).norm(); // Unit direction vector
        let o = ray.p;
        let c = pos;

        let udotoc = u.dot(o - c);
        let ocmag = (o - c).mag();
        let nabla = (udotoc * udotoc) - ((ocmag * ocmag) - (radius * radius));
        if nabla <= 0. {
            return None; // No solution / Tangential
        }
        let k1 = -(udotoc) + nabla.sqrt();
        let k2 = -(udotoc) - nabla.sqrt();
        let k = k1.min(k2);

        if k < 1. {
            return None; // Behind the start of the ray
        }

        let intersection: Vec3 = ray.p + (u * k);
        // Reflection
        let n = (intersection - pos).norm();
        let reflection: Vec3 = u - (((u * 2.).dot(n)) / (n.mag() * n.mag())) * n;
        // let reflection: Vec3 = u - (n * (u.dot(n) * 2.));
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

impl<T: Fn(f64) -> Vec3> Renderable for MovingSphere<T> {
    fn intersects(&self, l: &Ray, t: f64) -> Option<Hit> {
        Sphere::intersects((self.pos)(t), self.r, l, self.color, self.reflectivity)
    }
}

impl Renderable for Sphere {
    fn intersects(&self, r: &Ray, _t: f64) -> Option<Hit> {
        Sphere::intersects(self.pos, self.r, r, self.color, self.reflectivity)
    }
}
