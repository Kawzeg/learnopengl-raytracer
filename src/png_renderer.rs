use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

use crate::{renderer::Renderer, HEIGHT, WIDTH};

pub struct PngRenderer<T> where T: Renderer {
    renderer: T,
    start: f64,
    end: f64,
    steps: u64,
    stepsize: f64,
    current: u64,
    done: bool,
}

impl<T> PngRenderer<T> where T: Renderer {
    fn save_image(bytes: &Vec<u8>, step: u64, steps: u64) {
        let width = f64::log10(steps as f64).floor();
        if width > 5. {
            panic!("We can't handle more than 10000 images :(");
        }
        let pathname = format!("images/image-{:0>5}.png", step);
        let path = Path::new(&pathname);
        println!("Opening file: {:?}", path);
        let file = File::create(path).unwrap();
        println!("Rendering to: {:?}", file);
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, WIDTH.into(), HEIGHT.into());
        encoder.set_color(png::ColorType::RGBA);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(bytes).unwrap();
        println!("Render done");
    }
}

impl<T> PngRenderer<T> where T: Renderer {
    pub fn new(renderer: T, start: f64, end: f64) -> PngRenderer<T> {
        let fps = 30.;
        let interval = end - start;
        let steps = (interval * fps).round() as u64;
        let stepsize = interval / (steps as f64);
        PngRenderer {
            renderer, start, end, steps, current: 0, stepsize, done: false,
        }
    }
}

impl<T> Renderer for PngRenderer<T> where T: Renderer{
    fn render(&mut self, t: f64) -> (Vec<u8>, u16, u16) {
        println!("Step: {} of {}, done: {}", self.current, self.steps, self.done);
        if self.done {
            return (vec![0xAA; 4 * WIDTH as usize * HEIGHT as usize], WIDTH, HEIGHT);
        }
        let r = self.renderer.render(self.current as f64 * self.stepsize + self.start);
        PngRenderer::<T>::save_image(&r.0, self.current, self.steps);

        self.current += 1;
        if self.current > self.steps {
            self.current = 0;
            self.done = true;
        }
        r
    }
}