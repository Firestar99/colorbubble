#![no_std]

use glam::{Vec2, Vec4, Vec4Swizzles};
use spirv_std::image::Image2d;
use spirv_std::{Sampler, spirv};

pub struct VertexOutput {
    tex_coord: Vec2,
    vtx_color: Vec4,
}

pub struct FrameData {
    viewport: Vec4,
}

#[spirv(vertex)]
pub fn vs_main(
    #[spirv(descriptor_set = 0, binding = 0, uniform)] frame_data: &FrameData,
    position: Vec2,
    tex_coord: Vec2,
    vtx_color: Vec4,
    result: &mut VertexOutput,
    #[spirv(position)] gl_position: &mut Vec4,
) {
    result.tex_coord = tex_coord;
    result.vtx_color = vtx_color;
    *gl_position = Vec4::from((
        position * frame_data.viewport.zw() + frame_data.viewport.xy(),
        0.,
        1.,
    ));
}

#[spirv(fragment)]
pub fn fs_color(vertex: VertexOutput, color_out: &mut Vec4) {
    *color_out = vertex.vtx_color;
}

#[spirv(fragment)]
pub fn fs_texture(
    vertex: VertexOutput,
    #[spirv(descriptor_set = 1, binding = 0)] my_texture: &Image2d,
    #[spirv(descriptor_set = 1, binding = 1)] my_sampler: &Sampler,
    color_out: &mut Vec4,
) {
    *color_out = my_texture.sample(*my_sampler, vertex.tex_coord) * vertex.vtx_color;
    if color_out.w < 0.01 {
        spirv_std::arch::kill();
    }
}

#[spirv(fragment)]
pub fn fs_masked(
    vertex: VertexOutput,
    #[spirv(frag_coord)] frag_coord: Vec4,
    #[spirv(descriptor_set = 1, binding = 0)] my_texture: &Image2d,
    #[spirv(descriptor_set = 1, binding = 1)] my_sampler: &Sampler,
    #[spirv(descriptor_set = 2, binding = 0)] mask_texture: &Image2d,
    color_out: &mut Vec4,
) {
    *color_out = my_texture.sample(*my_sampler, vertex.tex_coord) * vertex.vtx_color;
    let mask = mask_texture.fetch(frag_coord.xy().as_uvec2()).x;
    if mask < 0.01 || color_out.w < 0.01 {
        spirv_std::arch::kill();
    }
}
