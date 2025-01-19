struct CameraUniform {
    view_projection: mat4x4<f32>,
    view_without_translation: mat4x4<f32>,
    projection: mat4x4<f32>
};
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uvw: vec3<f32>
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera.projection * camera.view_without_translation * vec4(model.position, 1.0);
    out.uvw = model.position;
    return out;
}

@group(0) @binding(0)
var t_sky: texture_cube<f32>;
@group(0) @binding(1)
var s_sky: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    //
    return textureSample(t_sky, s_sky, in.uvw);
}