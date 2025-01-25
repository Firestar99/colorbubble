use crate::rendering::framedata::{FrameDataBindGroupLayout, FrameDataBinding};
use crate::rendering::game_renderer::RenderConfig;
use bytemuck::{Pod, Zeroable};
use std::borrow::Cow;
use std::mem::offset_of;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{
    Buffer, BufferAddress, BufferUsages, RenderPass, RenderPipeline, VertexAttribute,
    VertexBufferLayout, VertexFormat, VertexStepMode,
};

pub const MAX_QUADS_PER_DRAW: u32 = 64;

#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct QuadVertex {
    pub position: glam::Vec2,
    pub tex_coord: glam::Vec2,
}

pub struct QuadVertexBuffer {
    pub buffer: Buffer,
    len: u32,
}

impl QuadVertexBuffer {
    /// vertices must be divisible by 4 and the vertices of a quad should be organized in a Z like pattern
    pub fn new(config: &RenderConfig, vertices: &[QuadVertex]) -> Self {
        assert!(vertices.len() < MAX_QUADS_PER_DRAW as usize);
        Self {
            buffer: config.device.create_buffer_init(&BufferInitDescriptor {
                label: Some("vertices"),
                contents: bytemuck::cast_slice(vertices),
                usage: BufferUsages::VERTEX,
            }),
            len: vertices.len() as u32,
        }
    }

    pub fn len(&self) -> u32 {
        self.len
    }
}

#[derive(Debug, Clone)]
pub struct QuadRenderer {
    pub config: RenderConfig,
    render_pipeline: RenderPipeline,
    index_buffer: Buffer,
}

impl QuadRenderer {
    pub fn new(config: &RenderConfig, frame_data_layout: &FrameDataBindGroupLayout) -> Self {
        let device = &config.device;
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("quad.wgsl"))),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&frame_data_layout.layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[VertexBufferLayout {
                    array_stride: size_of::<QuadVertex>() as u64,
                    step_mode: VertexStepMode::Vertex,
                    attributes: &[
                        VertexAttribute {
                            format: VertexFormat::Float32x2,
                            offset: offset_of!(QuadVertex, position) as BufferAddress,
                            shader_location: 0,
                        },
                        VertexAttribute {
                            format: VertexFormat::Float32x2,
                            offset: offset_of!(QuadVertex, tex_coord) as BufferAddress,
                            shader_location: 1,
                        },
                    ],
                }],
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

        let indices = (0..MAX_QUADS_PER_DRAW)
            .flat_map(|q| [0, 1, 2, 2, 1, 3].map(|i| i + q as u16))
            .collect::<Vec<_>>();
        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("global index buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: BufferUsages::INDEX,
        });

        Self {
            config: config.clone(),
            render_pipeline,
            index_buffer,
        }
    }

    pub fn draw(
        &self,
        rpass: &mut RenderPass,
        frame_data: &FrameDataBinding,
        vertices: QuadVertexBuffer,
    ) {
        rpass.set_pipeline(&self.render_pipeline);
        rpass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        rpass.set_vertex_buffer(0, vertices.buffer.slice(..));
        rpass.set_bind_group(0, Some(&frame_data.0), &[]);
        rpass.draw_indexed(0..vertices.len() / 4 * 6, 0, 0..1);
    }
}
