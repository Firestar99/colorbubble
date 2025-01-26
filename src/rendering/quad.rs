use crate::rendering::framedata::{FrameDataBindGroupLayout, FrameDataBinding};
use crate::rendering::game_renderer::RenderConfig;
use crate::rendering::quad_texture::{QuadTexture, QuadTextureBindGroupLayout};
use bytemuck::{Pod, Zeroable};
use std::borrow::Cow;
use std::mem::offset_of;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{
    Buffer, BufferAddress, BufferUsages, ColorTargetState, RenderPass, RenderPipeline,
    VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode,
};

pub const MAX_QUADS_PER_DRAW: u32 = 1024;

#[derive(Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct QuadVertex {
    pub position: glam::Vec2,
    pub tex_coord: glam::Vec2,
    pub vtx_color: glam::Vec4,
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
    pub frame_data_layout: FrameDataBindGroupLayout,
    pub texture_layout: QuadTextureBindGroupLayout,
    color_pipeline: RenderPipeline,
    texture_pipeline: RenderPipeline,
    index_buffer: Buffer,
}

impl QuadRenderer {
    pub fn load_texture(&self, path: &str) -> QuadTexture {
        let image = image::open(path).unwrap();
        QuadTexture::upload(&self.config, &self.texture_layout, &image.to_rgba8())
    }

    pub fn new(
        config: &RenderConfig,
        frame_data_layout: FrameDataBindGroupLayout,
        texture_layout: QuadTextureBindGroupLayout,
    ) -> Self {
        let device = &config.device;
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("quad.wgsl"))),
        });

        let vertex_state = wgpu::VertexState {
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
                    VertexAttribute {
                        format: VertexFormat::Float32x4,
                        offset: offset_of!(QuadVertex, vtx_color) as BufferAddress,
                        shader_location: 2,
                    },
                ],
            }],
            compilation_options: Default::default(),
        };
        let color_targets = [Some(ColorTargetState {
            format: config.swapchain_format,
            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
            write_mask: Default::default(),
        })];

        let color_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(
                &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[&frame_data_layout.layout],
                    push_constant_ranges: &[],
                }),
            ),
            vertex: vertex_state.clone(),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_color"),
                compilation_options: Default::default(),
                targets: &color_targets,
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let texture_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(
                &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[&frame_data_layout.layout, &texture_layout.layout],
                    push_constant_ranges: &[],
                }),
            ),
            vertex: vertex_state,
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_texture"),
                compilation_options: Default::default(),
                targets: &color_targets,
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let indices = (0..MAX_QUADS_PER_DRAW)
            .flat_map(|q| [0, 1, 2, 2, 1, 3].map(|i| i + (q as u16 * 4)))
            .collect::<Vec<_>>();
        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("global index buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: BufferUsages::INDEX,
        });

        Self {
            config: config.clone(),
            frame_data_layout,
            texture_layout,
            color_pipeline,
            texture_pipeline,
            index_buffer,
        }
    }

    pub fn draw_color(
        &self,
        rpass: &mut RenderPass,
        frame_data: &FrameDataBinding,
        vertices: &QuadVertexBuffer,
    ) {
        rpass.set_pipeline(&self.color_pipeline);
        self.draw_common(rpass, frame_data, vertices);
    }

    pub fn draw_texture(
        &self,
        rpass: &mut RenderPass,
        frame_data: &FrameDataBinding,
        vertices: &QuadVertexBuffer,
        texture: &QuadTexture,
    ) {
        rpass.set_pipeline(&self.texture_pipeline);
        rpass.set_bind_group(1, Some(&texture.bind), &[]);
        self.draw_common(rpass, frame_data, vertices);
    }

    pub fn draw_common(
        &self,
        rpass: &mut RenderPass,
        frame_data: &FrameDataBinding,
        vertices: &QuadVertexBuffer,
    ) {
        rpass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        rpass.set_vertex_buffer(0, vertices.buffer.slice(..));
        rpass.set_bind_group(0, Some(&frame_data.0), &[]);
        rpass.draw_indexed(0..vertices.len() / 4 * 6, 0, 0..1);
    }
}
