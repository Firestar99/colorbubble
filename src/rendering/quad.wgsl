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



@group(1)
@binding(0)
var my_texture: texture_2d<f32>;

@group(1)
@binding(1)
var my_sampler: sampler;

@fragment
fn fs_color(vertex: VertexOutput) -> @location(0) vec4<f32> {
	return vertex.vtx_color;
}

@fragment
fn fs_texture(vertex: VertexOutput) -> @location(0) vec4<f32> {
	var color = textureSample(my_texture, my_sampler, vertex.tex_coord) * vertex.vtx_color;
	if color.a < 0.01 {
		discard;
	}
	return color;
}



@group(2)
@binding(0)
var mask_texture: texture_2d<f32>;

@group(1)
@binding(1)
var mask_sampler: sampler;

@fragment
fn fs_masked(
	vertex: VertexOutput,
) -> @location(0) vec4<f32> {
	var color = textureSample(my_texture, my_sampler, vertex.tex_coord) * vertex.vtx_color;
	var mask = textureLoad(mask_texture, vec2u(vertex.position.xy), 0).x;
	if mask < 0.01 {
		discard;
	}
	if color.a < 0.01 {
		discard;
	}
	return color;
}
