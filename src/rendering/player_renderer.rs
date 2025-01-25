use crate::{
    main_loop::Player,
    rendering::quad::{QuadRenderer, QuadVertex, QuadVertexBuffer},
};
use glam::vec2;
use wgpu::RenderPass;

pub struct PlayerRenderer(QuadRenderer);

impl PlayerRenderer {
    pub fn new(quad: QuadRenderer) -> Self {
        Self(quad)
    }

    pub fn draw(&self, rpass: &mut RenderPass, player: &Player) {
        self.0.draw(
            rpass,
            QuadVertexBuffer::new(
                &self.0.config,
                &[
                    QuadVertex {
                        position: vec2(0., 0.) + player.pos,
                        tex_coord: vec2(0., 0.),
                    },
                    QuadVertex {
                        position: vec2(0., 1.) + player.pos,
                        tex_coord: vec2(0., 1.),
                    },
                    QuadVertex {
                        position: vec2(1., 0.) + player.pos,
                        tex_coord: vec2(1., 0.),
                    },
                    QuadVertex {
                        position: vec2(1., 1.) + player.pos,
                        tex_coord: vec2(1., 1.),
                    },
                ],
            ),
        )
    }
}
