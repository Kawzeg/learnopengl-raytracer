use crate::renderer::Renderer;

pub struct Raytracer {
}

impl Renderer for Raytracer {
    fn render(&mut self, t: f64) -> (Vec<u8>, u16, u16) {
        todo!()
    }
}