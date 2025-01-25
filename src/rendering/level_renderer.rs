use crate::level::Level;
use crate::rendering::framedata::{FrameDataBinding, VIEWPORT_SIZE};
use crate::rendering::quad::{QuadRenderer, QuadVertex, QuadVertexBuffer};
use crate::rendering::quad_texture::QuadTexture;
use glam::{vec2, vec4};
use std::sync::Arc;
use wgpu::RenderPass;

pub const DEBUG_DRAW_LEVEL: bool = true;

pub struct LevelRenderer {
    quad: QuadRenderer,
    loaded: Option<LoadedLevel>,
}

pub struct LoadedLevel {
    level: Arc<Level>,
    vertices: QuadVertexBuffer,
    texture: QuadTexture,
}

impl LevelRenderer {
    pub fn new(quad: QuadRenderer) -> Self {
        Self { quad, loaded: None }
    }

    pub fn load_level(&mut self, level: Arc<Level>) {
        let size = VIEWPORT_SIZE;
        let vertices = QuadVertexBuffer::new(
            &self.quad.config,
            &[
                QuadVertex {
                    position: vec2(0., 0.) * size,
                    tex_coord: vec2(0., 0.),
                    vtx_color: vec4(1., 1., 1., 1.),
                },
                QuadVertex {
                    position: vec2(0., 1.) * size,
                    tex_coord: vec2(0., 1.),
                    vtx_color: vec4(1., 1., 1., 1.),
                },
                QuadVertex {
                    position: vec2(1., 0.) * size,
                    tex_coord: vec2(1., 0.),
                    vtx_color: vec4(1., 1., 1., 1.),
                },
                QuadVertex {
                    position: vec2(1., 1.) * size,
                    tex_coord: vec2(1., 1.),
                    vtx_color: vec4(1., 1., 1., 1.),
                },
            ],
        );
        let texture = if DEBUG_DRAW_LEVEL {
            QuadTexture::upload(&self.quad.config, &self.quad.texture_layout, &level.image)
        } else {
            QuadTexture::new_cleared(&self.quad.config, &self.quad.texture_layout, level.size)
        };
        self.loaded = Some(LoadedLevel {
            vertices,
            level,
            texture,
        });
    }

    pub fn unload_level(&mut self) {
        self.loaded = None;
    }

    pub fn draw(&self, rpass: &mut RenderPass, frame_data: &FrameDataBinding) {
        if let Some(loaded) = &self.loaded {
            self.quad
                .draw_texture(rpass, frame_data, &loaded.vertices, &loaded.texture)
        }
    }
}
