use super::{Ray, hit::Hit};

pub(in super) trait Renderable {
    fn intersects(&self, l: &Ray, t: f64) -> Option<Hit>;
}