use crate::rendering::player_renderer::PlayerRenderer;
use wgpu::{Device, Queue, TextureFormat, TextureView};

#[derive(Debug, Clone)]
pub struct RenderConfig {
    pub device: Device,
    pub queue: Queue,
    pub swapchain_format: TextureFormat,
}

pub struct GameRenderer {
    config: RenderConfig,
    player: PlayerRenderer,
}

impl GameRenderer {
    pub fn new(config: RenderConfig) -> Self {
        Self {
            player: PlayerRenderer::new(&config),
            config,
        }
    }

    pub fn draw(&self, output: TextureView) {
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
                        load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            self.player.draw(&self.config, &mut rpass);
        }

        self.config.queue.submit(Some(encoder.finish()));
    }
}
