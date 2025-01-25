use crate::level::Level;
use crate::main_loop::DespawnedParticle;
use crate::rendering::framedata::{FrameData, FrameDataBinding, VIEWPORT_SIZE};
use crate::rendering::quad::{QuadRenderer, QuadVertex, QuadVertexBuffer};
use crate::rendering::quad_texture::QuadTexture;
use glam::{vec2, vec4, Vec2, Vec4};
use image::{ImageFormat, ImageReader};
use rand::{thread_rng, Rng};
use std::io::Cursor;
use std::sync::Arc;
use wgpu::util::{DeviceExt, TextureDataOrder};
use wgpu::{Extent3d, RenderPass, TextureDescriptor, TextureDimension, TextureUsages};

pub const DEBUG_DRAW_LEVEL: bool = true;

pub struct LevelRenderer {
    quad: QuadRenderer,
    splashes: Vec<QuadTexture>,
    loaded: Option<LoadedLevel>,
}

pub struct LoadedLevel {
    level: Arc<Level>,
    vertices: QuadVertexBuffer,
    level_texture: QuadTexture,
}

impl LevelRenderer {
    pub fn new(quad: QuadRenderer) -> anyhow::Result<Self> {
        let splashes = [
            include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/textures/colorsplash/1.png"
            ))
            .as_slice(),
            include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/textures/colorsplash/2.png"
            ))
            .as_slice(),
            include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/textures/colorsplash/3.png"
            ))
            .as_slice(),
        ];
        let splashes = splashes
            .iter()
            .map(|bytes| {
                let image = ImageReader::with_format(Cursor::new(*bytes), ImageFormat::Png)
                    .decode()?
                    .flipv()
                    .into_rgba8();
                Ok(QuadTexture::upload(
                    &quad.config,
                    &quad.texture_layout,
                    &image,
                ))
            })
            .collect::<anyhow::Result<Vec<_>>>()?;
        Ok(Self {
            quad,
            splashes,
            loaded: None,
        })
    }

    pub fn load_level(&mut self, level: Arc<Level>) {
        let size = VIEWPORT_SIZE;
        let vertices = QuadVertexBuffer::new(
            &self.quad.config,
            &[
                QuadVertex {
                    position: vec2(0., 0.) * size,
                    tex_coord: vec2(0., 0.),
                    vtx_color: vec4(1., 1., 1., 1.),
                },
                QuadVertex {
                    position: vec2(0., 1.) * size,
                    tex_coord: vec2(0., 1.),
                    vtx_color: vec4(1., 1., 1., 1.),
                },
                QuadVertex {
                    position: vec2(1., 0.) * size,
                    tex_coord: vec2(1., 0.),
                    vtx_color: vec4(1., 1., 1., 1.),
                },
                QuadVertex {
                    position: vec2(1., 1.) * size,
                    tex_coord: vec2(1., 1.),
                    vtx_color: vec4(1., 1., 1., 1.),
                },
            ],
        );

        let device = &self.quad.config.device;
        let image = &level.image;
        let texture_descriptor = TextureDescriptor {
            label: Some("Quad texture"),
            size: Extent3d {
                width: image.width(),
                height: image.height(),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: self.quad.config.swapchain_format,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        };
        let texture = if DEBUG_DRAW_LEVEL {
            device.create_texture_with_data(
                &self.quad.config.queue,
                &texture_descriptor,
                TextureDataOrder::MipMajor,
                &image.as_raw(),
            )
        } else {
            device.create_texture(&texture_descriptor)
        };
        let texture = QuadTexture::new(&self.quad.config, &self.quad.texture_layout, texture);
        self.loaded = Some(LoadedLevel {
            vertices,
            level,
            level_texture: texture,
        });
    }

    pub fn unload_level(&mut self) {
        self.loaded = None;
    }

    pub fn draw(&self, rpass: &mut RenderPass, frame_data: &FrameDataBinding) {
        if let Some(loaded) = &self.loaded {
            self.quad
                .draw_texture(rpass, frame_data, &loaded.vertices, &loaded.level_texture)
        }
    }

    pub fn draw_color_splashes(&mut self, bubbles: &[DespawnedParticle]) {
        if bubbles.is_empty() {
            return;
        }

        if let Some(loaded) = &self.loaded {
            let device = &self.quad.config.device;
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("splash draw"),
            });
            {
                let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &loaded.level_texture.texture_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });

                let frame_data = self.quad.frame_data_layout.create_bind_group(FrameData {
                    viewport: Vec4::from((Vec2::NEG_ONE, 1. / loaded.level.size.as_vec2() * 2.)),
                });

                let mut rng = thread_rng();
                for b in bubbles {
                    let size = 25.;
                    let pos = vec2(b.pos.x, VIEWPORT_SIZE.y - b.pos.y);
                    let texture = &self.splashes[rng.gen_range(0..self.splashes.len())];
                    // let vtx_color = Vec4::from((b.color, 1.));
                    let vtx_color = Vec4::new(0., 1., 0.5, 1.);
                    self.quad.draw_texture(
                        &mut rpass,
                        &frame_data,
                        &QuadVertexBuffer::new(
                            &self.quad.config,
                            &[
                                QuadVertex {
                                    position: vec2(-1., -1.) * size + pos,
                                    tex_coord: vec2(0., 0.),
                                    vtx_color,
                                },
                                QuadVertex {
                                    position: vec2(-1., 1.) * size + pos,
                                    tex_coord: vec2(0., 1.),
                                    vtx_color,
                                },
                                QuadVertex {
                                    position: vec2(1., -1.) * size + pos,
                                    tex_coord: vec2(1., 0.),
                                    vtx_color,
                                },
                                QuadVertex {
                                    position: vec2(1., 1.) * size + pos,
                                    tex_coord: vec2(1., 1.),
                                    vtx_color,
                                },
                            ],
                        ),
                        texture,
                    );
                }
            }

            self.quad.config.queue.submit(Some(encoder.finish()));
        }
    }
}
