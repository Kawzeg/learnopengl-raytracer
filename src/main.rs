mod raytracer;
use std::f64::consts::PI;

use png_renderer::PngRenderer;
use raytracer::Raytracer;

mod renderer;
use renderer::Renderer;

mod png_renderer;

mod util;

use miniquad::*;

const WIDTH: u16 = 400;
const HEIGHT: u16 = 300;

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
        let pixels: Vec<u8> = vec![0xFF; WIDTH as usize * HEIGHT as usize * 4];
        let texture = Texture::from_rgba8(ctx, WIDTH, HEIGHT, &pixels);
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

impl<R: Renderer> EventHandler for Stage<R> {
    fn update(&mut self, ctx: &mut Context) {
        let t = date::now();

        let (pixels, width, height) = self.renderer.render(t);
        self.bindings.images[0].update(ctx, &pixels);

        let r2 = width as f32 / height as f32;
        let (screen_width, screen_height) = ctx.screen_size();
        let r1 = screen_width / screen_height;

        let x0;
        let y0;
        if r2 > r1 {
            x0 = -1.;
            y0 = -r1 / r2;
        } else {
            y0 = -1.;
            x0 = -r2 / r1;
        }

        let vertices: [Vertex; 4] = [
            Vertex {
                pos: Vec2 { x: x0, y: y0 },
                uv: Vec2 { x: 0., y: 0. },
            },
            Vertex {
                pos: Vec2 { x: -x0, y: y0 },
                uv: Vec2 { x: 1., y: 0. },
            },
            Vertex {
                pos: Vec2 { x: -x0, y: -y0 },
                uv: Vec2 { x: 1., y: 1. },
            },
            Vertex {
                pos: Vec2 { x: x0, y: -y0 },
                uv: Vec2 { x: 0., y: 1. },
            },
        ];
        self.bindings.vertex_buffers[0].update(ctx, &vertices);

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

fn main() {
    let raytracer = Raytracer::new();
    miniquad::start(conf::Conf::default(), |mut ctx| {
        UserData::owning(Stage::new(&mut ctx, PngRenderer::new(raytracer, 0., PI * 2.)), ctx)
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
