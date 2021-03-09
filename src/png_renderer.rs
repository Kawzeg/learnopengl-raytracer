use crate::renderer::Renderer;

pub struct PngRenderer<T> where T: Renderer {
    renderer: T,
}

impl<T> PngRenderer<T> where T: Renderer {
    pub fn new(renderer: T) -> PngRenderer<T> {
        PngRenderer {
            renderer
        }
    }
}

impl<T> Renderer for PngRenderer<T> where T: Renderer{
    fn render(&mut self, t: f64) -> (Vec<u8>, u16, u16) {
        self.renderer.render(t)
    }
}