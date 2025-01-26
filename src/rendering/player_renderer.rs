use crate::entity::player::Player;
use crate::rendering::framedata::FrameDataBinding;
use crate::rendering::quad::{QuadRenderer, QuadVertex, QuadVertexBuffer};
use glam::{vec2, vec4};
use wgpu::RenderPass;

pub struct PlayerRenderer(QuadRenderer);

impl PlayerRenderer {
    pub fn new(quad: QuadRenderer) -> Self {
        Self(quad)
    }

    pub fn draw(&self, rpass: &mut RenderPass, frame_data: &FrameDataBinding, player: &Player) {
        if player.hidden {
            return;
        }

        let size = vec2(28., 28.);
        let vtx_color = vec4(1., 1., 1., 1.);
        self.0.draw_color(
            rpass,
            frame_data,
            &QuadVertexBuffer::new(
                &self.0.config,
                &[
                    QuadVertex {
                        position: vec2(0., 0.) * size + player.pos,
                        tex_coord: vec2(0., 0.),
                        vtx_color,
                    },
                    QuadVertex {
                        position: vec2(0., 1.) * size + player.pos,
                        tex_coord: vec2(0., 1.),
                        vtx_color,
                    },
                    QuadVertex {
                        position: vec2(1., 0.) * size + player.pos,
                        tex_coord: vec2(1., 0.),
                        vtx_color,
                    },
                    QuadVertex {
                        position: vec2(1., 1.) * size + player.pos,
                        tex_coord: vec2(1., 1.),
                        vtx_color,
                    },
                ],
            ),
        )
    }
}
