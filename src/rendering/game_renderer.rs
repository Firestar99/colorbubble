use super::splash_renderer::SplashRenderer;
use crate::entity::game::Game;
use crate::rendering::bubble_renderer::BubbleRenderer;
use crate::rendering::framedata::{get_viewport, FrameData, FrameDataBindGroupLayout};
use crate::rendering::level_renderer::LevelRenderer;
use crate::rendering::player_renderer::PlayerRenderer;
use crate::rendering::quad::QuadRenderer;
use crate::rendering::quad_texture::QuadTextureBindGroupLayout;
use wgpu::{Device, Queue, TextureFormat, TextureView};

#[derive(Debug, Clone)]
pub struct RenderConfig {
    pub device: Device,
    pub queue: Queue,
    pub swapchain_format: TextureFormat,
}

pub struct GameRenderer {
    pub config: RenderConfig,
    pub quad: QuadRenderer,
    pub player: PlayerRenderer,
    pub splash: SplashRenderer,
    pub bubble: BubbleRenderer,
    pub level: LevelRenderer,
}

impl GameRenderer {
    pub fn new(config: &RenderConfig) -> anyhow::Result<Self> {
        let frame_data_layout = FrameDataBindGroupLayout::new(config);
        let quad_texture_layout = QuadTextureBindGroupLayout::new(config);
        let quad = QuadRenderer::new(config, frame_data_layout, quad_texture_layout);
        Ok(Self {
            player: PlayerRenderer::new(quad.clone()),
            level: LevelRenderer::new(quad.clone())?,
            splash: SplashRenderer::new(quad.clone()),
            bubble: BubbleRenderer::new(quad.clone()),
            quad,
            config: config.clone(),
        })
    }

    pub fn draw<'a>(&self, game: &Game, output: TextureView) {
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

            let frame_data = self.quad.frame_data_layout.create_bind_group(FrameData {
                viewport: get_viewport(game.level.size, game.player.pos),
            });
            self.level.draw(&mut rpass, &frame_data);
            self.player.draw(&mut rpass, &frame_data, &game.player);
            game.portal
                .render(&mut rpass, &frame_data, &self.quad, &game.player);
            self.splash
                .draw(&mut rpass, &frame_data, game.splashes.as_slice());
            self.bubble
                .draw(&mut rpass, &frame_data, game.player_bubble.as_slice());
        }

        self.config.queue.submit(Some(encoder.finish()));
    }
}
