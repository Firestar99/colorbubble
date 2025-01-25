use super::quad_texture::QuadTexture;
use crate::main_loop::ParticleRenderData;
use crate::rendering::framedata::FrameDataBinding;
use crate::rendering::quad::{QuadRenderer, QuadVertex, QuadVertexBuffer};
use glam::{vec2, Vec4};
use wgpu::RenderPass;

pub struct ParticleRenderer(QuadRenderer);

impl ParticleRenderer {
    pub fn new(quad: QuadRenderer) -> Self {
        Self(quad)
    }

    pub fn load_texture(&self, path: &str) -> QuadTexture {
        let image = image::open(path).unwrap();
        QuadTexture::upload(&self.0.config, &self.0.texture_layout, &image.to_rgba8())
    }

    pub fn draw(
        &self,
        rpass: &mut RenderPass,
        frame_data: &FrameDataBinding,
        particle: ParticleRenderData,
    ) {
        let size = vec2(28., 28.);
        self.0.draw_texture(
            rpass,
            frame_data,
            &QuadVertexBuffer::new(
                &self.0.config,
                &[
                    QuadVertex {
                        position: vec2(0., 0.) * size + particle.pos,
                        tex_coord: vec2(0., 0.),
                        vtx_color: Vec4::ONE,
                    },
                    QuadVertex {
                        position: vec2(0., 1.) * size + particle.pos,
                        tex_coord: vec2(0., 1.),
                        vtx_color: Vec4::ONE,
                    },
                    QuadVertex {
                        position: vec2(1., 0.) * size + particle.pos,
                        tex_coord: vec2(1., 0.),
                        vtx_color: Vec4::ONE,
                    },
                    QuadVertex {
                        position: vec2(1., 1.) * size + particle.pos,
                        tex_coord: vec2(1., 1.),
                        vtx_color: Vec4::ONE,
                    },
                ],
            ),
            particle.img,
        )
    }
}
