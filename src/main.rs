use std::f64::consts::{self, PI};

use miniquad::*;

#[repr(C)]
struct Vec2 {
    x: f32,
    y: f32,
}
#[repr(C)]
struct Vertex {
    pos: Vec2,
    uv: Vec2,
}

const TEXTURE_WIDTH: u16 = 800;
const TEXTURE_HEIGHT: u16 = 600;
const SECONDS: f64 = 5.;
const TEXTURE_SIZE: usize = 4 * (TEXTURE_WIDTH as usize) * (TEXTURE_HEIGHT as usize);
const CHUNK_WIDTH_R: u16 = TEXTURE_WIDTH - 1;
const CHUNK_WIDTH_B: u16 = TEXTURE_WIDTH + 1;

struct Stage<R: Renderer> {
    pipeline: Pipeline,
    bindings: Bindings,
    renderer: R,
}

impl<R: Renderer> Stage<R> {
    pub fn new(ctx: &mut Context, renderer: R) -> Stage<R> {
        #[rustfmt::skip]
        let vertices: [Vertex; 4] = [
            Vertex { pos: Vec2 { x: -1., y: -1. }, uv: Vec2 { x: 0., y: 0. } },
            Vertex { pos: Vec2 { x:  1., y: -1. }, uv: Vec2 { x: 1., y: 0. } },
            Vertex { pos: Vec2 { x:  1., y:  1. }, uv: Vec2 { x: 1., y: 1. } },
            Vertex { pos: Vec2 { x: -1., y:  1. }, uv: Vec2 { x: 0., y: 1. } },
        ];
        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);

        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

        #[rustfmt::skip]
        let pixels: Vec<u8> = vec![0xFF; TEXTURE_SIZE];
        let texture = Texture::from_rgba8(ctx, TEXTURE_WIDTH, TEXTURE_HEIGHT, &pixels);
        texture.set_filter(ctx, FilterMode::Nearest);

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![texture],
        };

        let shader = Shader::new(ctx, shader::VERTEX, shader::FRAGMENT, shader::meta()).unwrap();

        let pipeline = Pipeline::new(
            ctx,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float2),
                VertexAttribute::new("uv", VertexFormat::Float2),
            ],
            shader,
        );

        Stage {
            pipeline,
            bindings,
            renderer,
        }
    }
}

pub trait Renderer {
    fn render(&mut self, t: f64) -> Vec<u8>; // Renderer draws
}

impl<R: Renderer> EventHandler for Stage<R> {
    fn update(&mut self, ctx: &mut Context) {
        let t = date::now();

        let pixels = self.renderer.render(t);
        let texture = Texture::from_rgba8(ctx, TEXTURE_WIDTH, TEXTURE_HEIGHT, &pixels);
        texture.set_filter(ctx, FilterMode::Nearest);
        self.bindings.images = vec![texture];

        let dt = date::now() - t;
        println!("Update took {}s", dt);
    }

    fn draw(&mut self, ctx: &mut Context) {
        let t = date::now();
        ctx.begin_default_pass(Default::default());

        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);
        ctx.draw(0, 6, 1);
        ctx.end_render_pass();

        ctx.commit_frame();
        let dt = date::now() - t;
        println!("Render took {}s", dt);
    }
}

struct SineRenderer {}

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
    fn render(&mut self, t: f64) -> Vec<u8> {
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
        pixels
    }
}

fn main() {
    miniquad::start(conf::Conf::default(), |mut ctx| {
        UserData::owning(Stage::new(&mut ctx, SineRenderer {}), ctx)
    });
}

mod shader {
    use miniquad::*;

    pub const VERTEX: &str = r#"#version 100
    attribute vec2 pos;
    attribute vec2 uv;
    varying lowp vec2 texcoord;
    void main() {
        gl_Position = vec4(pos, 0, 1);
        texcoord = uv;
    }"#;

    pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec2 texcoord;
    uniform sampler2D tex;
    void main() {
        gl_FragColor = texture2D(tex, texcoord);
    }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec!["tex".to_string()],
            uniforms: UniformBlockLayout { uniforms: vec![] },
        }
    }
}
