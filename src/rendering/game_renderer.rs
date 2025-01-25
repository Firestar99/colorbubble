use crate::main_loop::Player;
use crate::rendering::framedata::{
    get_viewport, FrameData, FrameDataBindGroupLayout, VIEWPORT_SIZE,
};
use crate::rendering::player_renderer::PlayerRenderer;
use crate::rendering::quad::QuadRenderer;
use wgpu::{Device, Queue, TextureFormat, TextureView};

#[derive(Debug, Clone)]
pub struct RenderConfig {
    pub device: Device,
    pub queue: Queue,
    pub swapchain_format: TextureFormat,
}

pub struct GameRenderer {
    config: RenderConfig,
    frame_data_layout: FrameDataBindGroupLayout,
    quad: QuadRenderer,
    player: PlayerRenderer,
}

impl GameRenderer {
    pub fn new(config: &RenderConfig) -> Self {
        let frame_data_layout = FrameDataBindGroupLayout::new(config);
        let quad = QuadRenderer::new(config, &frame_data_layout);
        Self {
            player: PlayerRenderer::new(quad.clone()),
            frame_data_layout,
            quad,
            config: config.clone(),
        }
    }

    pub fn draw(&self, player: &Player, output: TextureView) {
        let device = &self.config.device;
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("main draw"),
        });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &output,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            let frame_data = self.frame_data_layout.create_bind_group(FrameData {
                viewport: get_viewport(VIEWPORT_SIZE.as_uvec2(), player.pos),
            });
            self.player.draw(&mut rpass, &frame_data, &player);
        }

        self.config.queue.submit(Some(encoder.finish()));
    }
}
