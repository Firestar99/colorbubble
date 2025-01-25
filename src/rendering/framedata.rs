use crate::rendering::game_renderer::RenderConfig;
use bytemuck::{Pod, Zeroable};
use glam::{vec2, UVec2, Vec2, Vec4};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, BufferBinding, BufferBindingType,
    BufferUsages, ShaderStages,
};

pub const VIEWPORT_SIZE: Vec2 = vec2(800., 600.);

#[repr(C)]
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
pub struct FrameData {
    pub viewport: Vec4,
}

pub fn get_viewport(level_size: UVec2, player: Vec2) -> Vec4 {
    let position = player.clamp(Vec2::ZERO, level_size.as_vec2() - VIEWPORT_SIZE);
    Vec4::from((position, 1. / VIEWPORT_SIZE))
}

pub struct FrameDataBindGroupLayout {
    pub config: RenderConfig,
    pub layout: BindGroupLayout,
}

pub struct FrameDataBinding(pub BindGroup);

impl FrameDataBindGroupLayout {
    pub fn new(config: &RenderConfig) -> Self {
        let device = &config.device;
        Self {
            config: config.clone(),
            layout: device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("FrameData BindGroupLayout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX_FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            }),
        }
    }

    pub fn create_bind_group(&self, frame_data: FrameData) -> FrameDataBinding {
        let device = &self.config.device;
        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("FrammeData Buffer"),
            contents: bytemuck::cast_slice(&[frame_data]),
            usage: BufferUsages::UNIFORM,
        });
        FrameDataBinding(device.create_bind_group(&BindGroupDescriptor {
            label: Some("FrameData Bind Group"),
            layout: &self.layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: BindingResource::Buffer(BufferBinding {
                    buffer: &buffer,
                    offset: 0,
                    size: None,
                }),
            }],
        }))
    }
}
