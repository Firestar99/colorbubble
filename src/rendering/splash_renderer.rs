use super::quad_texture::QuadTexture;
use crate::entity::splash::Splash;
use crate::rendering::framedata::FrameDataBinding;
use crate::rendering::quad::{QuadRenderer, QuadVertex, QuadVertexBuffer};
use glam::{Mat2, vec2};
use wgpu::RenderPass;

pub struct SplashRenderer {
    pub quad: QuadRenderer,
    pub splash_texture: QuadTexture,
}

impl SplashRenderer {
    pub fn new(quad: QuadRenderer) -> Self {
        Self {
            splash_texture: quad.load_texture(
                include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/Splash.png"))
                    .as_slice(),
            ),
            quad,
        }
    }

    pub fn draw(&self, rpass: &mut RenderPass, frame_data: &FrameDataBinding, splashes: &[Splash]) {
        if splashes.is_empty() {
            return;
        }

        let vertices = QuadVertexBuffer::new(
            &self.quad.config,
            &splashes
                .iter()
                .flat_map(|splash| {
                    let size = vec2(7., 7.);
                    let dir = splash.vel.normalize();
                    let rot = Mat2::from_cols_array_2d(&[[dir.y, -dir.x], [dir.x, dir.y]]);
                    let vtx_color = splash.color;
                    [
                        QuadVertex {
                            position: rot * vec2(-1., -1.) * size + splash.pos,
                            tex_coord: vec2(0., 0.),
                            vtx_color,
                        },
                        QuadVertex {
                            position: rot * vec2(-1., 1.) * size + splash.pos,
                            tex_coord: vec2(0., 1.),
                            vtx_color,
                        },
                        QuadVertex {
                            position: rot * vec2(1., -1.) * size + splash.pos,
                            tex_coord: vec2(1., 0.),
                            vtx_color,
                        },
                        QuadVertex {
                            position: rot * vec2(1., 1.) * size + splash.pos,
                            tex_coord: vec2(1., 1.),
                            vtx_color,
                        },
                    ]
                })
                .collect::<Vec<_>>(),
        );
        self.quad
            .draw_texture(rpass, frame_data, &vertices, &self.splash_texture)
    }
}
