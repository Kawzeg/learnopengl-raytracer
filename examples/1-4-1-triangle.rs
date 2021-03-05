use std::mem::size_of;

use miniquad::*;

#[repr(C)]
struct Vec2 {
    x: f32,
    y: f32,
    z: f32,
}

struct Stage {
    pipeline: Pipeline,
    bindings: Bindings,
}

impl Stage {
    pub fn new(ctx: &mut Context) -> Stage {
        // Compile Shaders
        let shader = Shader::new(ctx, shader::VERTEX, shader::FRAGMENT, shader::meta()).unwrap();

        // Vertices of a triangle
        let vertices: [Vec2; 3] = [
            Vec2 { x: -0.5, y: -0.5, z: 0.0 }, // Bottom left
            Vec2 { x:  0.5, y: -0.5, z: 0.0 }, // Bottom right
            Vec2 { x:  0.0, y:  0.5, z: 0.0 }, // Top
        ];
        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);

        // Draw triangle in order
        let indices: [u16; 3] = [0,1,2];
        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

        let pipeline = Pipeline::new(
            ctx,
            &[
                BufferLayout::default()
            ],
            &[
                VertexAttribute::new("pos", VertexFormat::Float3)
            ],
            shader);
        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![],
        };

        Stage { pipeline, bindings }
    }
}

impl EventHandler for Stage {
    fn update(&mut self, _ctx: &mut Context) {
    }

    fn draw(&mut self, ctx: &mut Context) {
        ctx.begin_default_pass(Default::default());
        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);
        ctx.draw(0, 3, 1);
        ctx.end_render_pass();
        ctx.commit_frame();
    }
}

fn main() {
    miniquad::start(conf::Conf::default(), |mut ctx| {
        UserData::owning(Stage::new(&mut ctx), ctx)
    });
}

mod shader {
    use miniquad::*;

    pub const VERTEX: &str = r#"#version 100
    attribute vec3 pos;

    void main() {
        gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
    }"#;

    pub const FRAGMENT: &str = r#"#version 100
    void main() {
        gl_FragColor = vec4(1.0, 0.5, 0.2, 1.0);
    }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images:  vec![],
            uniforms: UniformBlockLayout {
                uniforms: vec![]
            },
        }
    }
}