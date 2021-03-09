use super::{Ray, hit::Hit};

pub(in super) trait Renderable {
    /// Returns false and Vec3::NULL if no intersection
    /// Returns true and a reflected ray if yes intersection
    fn intersects(&self, l: &Ray) -> Option<Hit>;
}