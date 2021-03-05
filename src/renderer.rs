use std::f64::consts::PI;

pub trait Renderer {
    /// Returns a texture as rgba pixel array with width and height
    fn render(&mut self, t: f64) -> (Vec<u8>, u16, u16);
}

const TEXTURE_WIDTH: u16 = 800;
const TEXTURE_HEIGHT: u16 = 600;
const SECONDS: f64 = 5.;
const TEXTURE_SIZE: usize = 4 * (TEXTURE_WIDTH as usize) * (TEXTURE_HEIGHT as usize);
const CHUNK_WIDTH_R: u16 = TEXTURE_WIDTH - 1;
const CHUNK_WIDTH_B: u16 = TEXTURE_WIDTH + 1;
pub struct SineRenderer {}

impl SineRenderer {
    fn clamp<T: Into<f64>, U: Into<f64>>(x: T, y: U) -> f64 {
        let a: f64 = x.into();
        let b: f64 = y.into();
        (a % b) / b
    }

    fn b(t: f64, i: usize) -> u8 {
        let progress = SineRenderer::clamp(i as f64, CHUNK_WIDTH_B);
        let tm = SineRenderer::clamp(t, SECONDS);

        let x = (tm - progress) * PI * 2.;

        let r = ((x.cos() + 1.) / 2. * 255.).round() as u8;
        return r;
    }

    fn r(t: f64, i: usize) -> u8 {
        let progress = SineRenderer::clamp(i as f64, CHUNK_WIDTH_R);
        let tm = SineRenderer::clamp(t, SECONDS);

        let x = (tm + progress) * PI * 2.;

        // let r = (((i as f64) / TEXTURE_WIDTH as f64).floor() / (TEXTURE_HEIGHT as f64) * 255.).round() as u8;
        let r = ((x.cos() + 1.) / 2. * 255.).round() as u8;
        return r;
    }
}

impl Renderer for SineRenderer {
    fn render(&mut self, t: f64) -> (Vec<u8>, u16, u16) {
        let mut pixels: Vec<u8> = vec![0x00; TEXTURE_SIZE];
        for i in (0..TEXTURE_SIZE).step_by(4) {
            pixels[i] = SineRenderer::r(t, ((i as f64) / 4.).floor() as usize); // Red
        }

        for i in (2..TEXTURE_SIZE).step_by(4) {
            pixels[i] = SineRenderer::b(t, ((i as f64) / 4.).floor() as usize); // Blue
        }

        for i in (3..TEXTURE_SIZE).step_by(4) {
            pixels[i] = 0xFF; // Alpha
        }
        (pixels, TEXTURE_WIDTH, TEXTURE_HEIGHT)
    }
}