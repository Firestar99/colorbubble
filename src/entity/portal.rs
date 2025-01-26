use crate::entity::player::Player;
use crate::rendering::framedata::FrameDataBinding;
use crate::rendering::quad::{QuadRenderer, QuadVertex, QuadVertexBuffer};
use glam::{vec2, Vec2, Vec4};
use wgpu::RenderPass;

#[derive(Debug, Copy, Clone)]
pub struct Portal {
    pub pos: Vec2,
    pub tick: Option<u32>,
}

impl Portal {
    pub fn new(pos: Vec2) -> Self {
        Self { pos, tick: None }
    }

    pub fn update(&mut self, player: &mut Player) {
        if let Some(tick) = &mut self.tick {
            *tick -= 1;
        } else {
            if player.pos.distance(self.pos) < 15. {
                player.hidden = true;
                self.tick = Some(30);
            }
        }
    }

    pub fn jump_to_next_level(&self) -> bool {
        self.tick == Some(0)
    }

    pub fn render(
        &self,
        rpass: &mut RenderPass,
        frame_data_binding: &FrameDataBinding,
        quad: &QuadRenderer,
        player: &Player,
    ) {
        let white = Vec4::splat(1.0);
        let color = player.color();

        let add = if let Some(tick) = self.tick {
            tick as f32 / 5.
        } else {
            0.
        };
        let add2 = add * 2.;

        quad.draw_color(
            rpass,
            frame_data_binding,
            &QuadVertexBuffer::new(
                &quad.config,
                &[
                    QuadVertex {
                        position: vec2(-15. + add, 40. + add2) + self.pos,
                        tex_coord: Default::default(),
                        vtx_color: white,
                    },
                    QuadVertex {
                        position: vec2(15. - add, 40. + add2) + self.pos,
                        tex_coord: Default::default(),
                        vtx_color: white,
                    },
                    QuadVertex {
                        position: vec2(-15. + add, 0.) + self.pos,
                        tex_coord: Default::default(),
                        vtx_color: white,
                    },
                    QuadVertex {
                        position: vec2(15. - add, 0.) + self.pos,
                        tex_coord: Default::default(),
                        vtx_color: white,
                    },
                    QuadVertex {
                        position: vec2(-12. + add, 37. + add2) + self.pos,
                        tex_coord: Default::default(),
                        vtx_color: color,
                    },
                    QuadVertex {
                        position: vec2(12. - add, 37. + add2) + self.pos,
                        tex_coord: Default::default(),
                        vtx_color: color,
                    },
                    QuadVertex {
                        position: vec2(-12. + add, 3.) + self.pos,
                        tex_coord: Default::default(),
                        vtx_color: color,
                    },
                    QuadVertex {
                        position: vec2(12. - add, 3.) + self.pos,
                        tex_coord: Default::default(),
                        vtx_color: color,
                    },
                ],
            ),
        );
    }
}
