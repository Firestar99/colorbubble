#![no_std]

use glam::{Vec2, Vec4, Vec4Swizzles};
use spirv_std::image::sample_with::lod;
use spirv_std::image::{Image2d, ImageWithMethods};
use spirv_std::{Sampler, spirv};

pub struct FrameData {
    viewport: Vec4,
}

#[spirv(vertex)]
pub fn vs_main(
    #[spirv(descriptor_set = 0, binding = 0, uniform)] frame_data: &FrameData,
    position: Vec2,
    tex_coord: Vec2,
    color: Vec4,
    vtx_color: &mut Vec4,
    vtx_tex_coord: &mut Vec2,
    #[spirv(position)] gl_position: &mut Vec4,
) {
    *vtx_tex_coord = tex_coord;
    *vtx_color = color;
    let mut position = Vec4::from((
        position * frame_data.viewport.zw() + frame_data.viewport.xy(),
        0.,
        1.,
    ));
    position.y = -position.y;
    *gl_position = position;
}

#[spirv(fragment)]
pub fn fs_color(vtx_color: Vec4, _vtx_tex_coord: Vec2, color_out: &mut Vec4) {
    *color_out = Vec4::from(vtx_color);
}

#[spirv(fragment)]
pub fn fs_texture(
    vtx_color: Vec4,
    vtx_tex_coord: Vec2,
    #[spirv(descriptor_set = 1, binding = 0)] my_texture: &Image2d,
    #[spirv(descriptor_set = 1, binding = 1)] my_sampler: &Sampler,
    color_out: &mut Vec4,
) {
    *color_out = my_texture.sample(*my_sampler, vtx_tex_coord) * vtx_color;
    if color_out.w < 0.01 {
        spirv_std::arch::kill();
    }
}

#[spirv(fragment)]
pub fn fs_masked(
    vtx_color: Vec4,
    vtx_tex_coord: Vec2,
    #[spirv(frag_coord)] frag_coord: Vec4,
    #[spirv(descriptor_set = 1, binding = 0)] my_texture: &Image2d,
    #[spirv(descriptor_set = 1, binding = 1)] my_sampler: &Sampler,
    #[spirv(descriptor_set = 2, binding = 0)] mask_texture: &Image2d,
    color_out: &mut Vec4,
) {
    *color_out = my_texture.sample(*my_sampler, vtx_tex_coord) * vtx_color;
    let mask = mask_texture
        .fetch_with(frag_coord.xy().as_uvec2(), lod(0))
        .x;
    if mask < 0.01 || color_out.w < 0.01 {
        spirv_std::arch::kill();
    }
}
