use super::{Ray, Rgb};

pub struct Hit {
    pub(in super) reflection: Ray,
    pub(in super) color: Rgb,
    pub reflectivity: f64,
}

impl Hit {
}
