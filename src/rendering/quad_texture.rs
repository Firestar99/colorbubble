use crate::rendering::game_renderer::RenderConfig;
use image::RgbaImage;
use wgpu::util::{DeviceExt, TextureDataOrder};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, Extent3d, FilterMode, Sampler,
    SamplerBindingType, SamplerDescriptor, ShaderStages, TextureDescriptor, TextureDimension,
    TextureFormat, TextureSampleType, TextureUsages, TextureViewDescriptor, TextureViewDimension,
};

#[derive(Debug, Clone)]
pub struct QuadTextureBindGroupLayout {
    pub layout: BindGroupLayout,
    pub sampler: Sampler,
}

impl QuadTextureBindGroupLayout {
    pub fn new(config: &RenderConfig) -> Self {
        let device = &config.device;
        let layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Quad texture bind group layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::VERTEX_FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::default(),
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::VERTEX_FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        let sampler = device.create_sampler(&SamplerDescriptor {
            label: Some("Quad texture sampler"),
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });
        Self { layout, sampler }
    }
}

#[derive(Debug, Clone)]
pub struct QuadTexture(pub BindGroup);

impl QuadTexture {
    pub fn upload(
        config: &RenderConfig,
        layout: &QuadTextureBindGroupLayout,
        image: &RgbaImage,
    ) -> Self {
        let device = &config.device;
        let texture = device.create_texture_with_data(
            &config.queue,
            &TextureDescriptor {
                label: Some("Quad texture"),
                size: Extent3d {
                    width: image.width(),
                    height: image.height(),
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Rgba8UnormSrgb,
                usage: TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            },
            TextureDataOrder::MipMajor,
            &image.as_raw(),
        );
        let texture_view = texture.create_view(&TextureViewDescriptor::default());
        Self(device.create_bind_group(&BindGroupDescriptor {
            label: Some("Quad texture bind group"),
            layout: &layout.layout,
            entries: &[
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(&texture_view),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::Sampler(&layout.sampler),
                },
            ],
        }))
    }
}
