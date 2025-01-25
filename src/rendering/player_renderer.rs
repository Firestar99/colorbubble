use crate::rendering::framedata::FrameDataBinding;
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

    pub fn draw(&self, rpass: &mut RenderPass, frame_data: &FrameDataBinding, player: &Player) {
        let size = vec2(28., 28.);

        self.0.draw(
            rpass,
            frame_data,
            QuadVertexBuffer::new(
                &self.0.config,
                &[
                    QuadVertex {
                        position: vec2(0., 0.) * size + player.pos,
                        tex_coord: vec2(0., 0.),
                    },
                    QuadVertex {
                        position: vec2(0., 1.) * size + player.pos,
                        tex_coord: vec2(0., 1.),
                    },
                    QuadVertex {
                        position: vec2(1., 0.) * size + player.pos,
                        tex_coord: vec2(1., 0.),
                    },
                    QuadVertex {
                        position: vec2(1., 1.) * size + player.pos,
                        tex_coord: vec2(1., 1.),
                    },
                ],
            ),
        )
    }
}
