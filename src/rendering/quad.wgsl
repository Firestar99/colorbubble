struct VertexOutput {
    @location(0) tex_coord: vec2<f32>,
    @location(1) vtx_color: vec4<f32>,
    @builtin(position) position: vec4<f32>,
};

struct FrameData {
	viewport: vec4<f32>,
};

@group(0)
@binding(0)
var<uniform> frame_data: FrameData;

@vertex
fn vs_main(
    @location(0) position: vec2<f32>,
    @location(1) tex_coord: vec2<f32>,
    @location(2) vtx_color: vec4<f32>,
) -> VertexOutput {
    var result: VertexOutput;
    result.tex_coord = tex_coord;
    result.position = vec4(position * frame_data.viewport.zw + frame_data.viewport.xy, 0., 1.);
    result.vtx_color = vtx_color;
    return result;
}

@fragment
fn fs_color(vertex: VertexOutput) -> @location(0) vec4<f32> {
	return vertex.vtx_color;
}

@group(1)
@binding(1)
var my_texture: texture_2d<f32>;

@group(1)
@binding(2)
var my_sampler: sampler;

@fragment
fn fs_texture(vertex: VertexOutput) -> @location(0) vec4<f32> {
	return textureSample(my_texture, my_sampler, vertex.tex_coord) * vertex.vtx_color;
}
