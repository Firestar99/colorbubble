use super::quad_texture::QuadTexture;
use crate::entity::splash::Splash;
use crate::rendering::framedata::FrameDataBinding;
use crate::rendering::quad::{QuadRenderer, QuadVertex, QuadVertexBuffer};
use glam::{vec2, Vec4};
use wgpu::RenderPass;

pub struct SplashRenderer {
    pub quad: QuadRenderer,
    pub splash_texture: QuadTexture,
}

impl SplashRenderer {
    pub fn new(quad: QuadRenderer) -> Self {
        Self {
            splash_texture: quad.load_texture("assets/Splash.png"),
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
                    let size = vec2(28., 28.);
                    let vtx_color = Vec4::splat(1.); // todo
                    [
                        QuadVertex {
                            position: vec2(0., 0.) * size + splash.pos,
                            tex_coord: vec2(0., 0.),
                            vtx_color,
                        },
                        QuadVertex {
                            position: vec2(0., 1.) * size + splash.pos,
                            tex_coord: vec2(0., 1.),
                            vtx_color,
                        },
                        QuadVertex {
                            position: vec2(1., 0.) * size + splash.pos,
                            tex_coord: vec2(1., 0.),
                            vtx_color,
                        },
                        QuadVertex {
                            position: vec2(1., 1.) * size + splash.pos,
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
