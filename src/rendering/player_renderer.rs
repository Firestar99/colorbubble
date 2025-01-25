use crate::rendering::game_renderer::RenderConfig;
use std::borrow::Cow;
use wgpu::{RenderPass, RenderPipeline};

pub struct PlayerRenderer {
    render_pipeline: RenderPipeline,
}

impl PlayerRenderer {
    pub fn new(config: &RenderConfig) -> Self {
        let device = &config.device;
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("trongle.wgsl"))),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(config.swapchain_format.into())],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Self {
            render_pipeline,
        }
    }

    pub fn draw(&self, _config: &RenderConfig, rpass: &mut RenderPass) {
        rpass.set_pipeline(&self.render_pipeline);
        rpass.draw(0..3, 0..1);
    }
}
