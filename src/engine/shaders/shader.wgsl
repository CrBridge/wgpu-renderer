struct CameraUniform {
    view_projection: mat4x4<f32>,
    view_without_translation: mat4x4<f32>,
    projection: mat4x4<f32>
};
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct LightUniform {
    direction: vec3<f32>,
    color: vec3<f32>,
};
@group(2) @binding(0)
var<uniform> light: LightUniform;

struct ModelPush { 
    model: mat4x4<f32> 
}
var<push_constant> push: ModelPush;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) normal: vec3<f32>
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) vertex_light: f32
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    let light_direction = normalize(light.direction);
    let normal_world_space = normalize((push.model * vec4<f32>(model.normal, 0)).xyz);
    let light_intensity = max(dot(normal_world_space, light_direction), 0.0);
    out.vertex_light = light_intensity;

    out.uv = model.uv;
    out.normal = model.normal;
    out.clip_position = camera.view_projection * push.model * vec4<f32>(model.position, 1.0);
    return out;
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let texture_color = textureSample(t_diffuse, s_diffuse, in.uv);
    let ambient = light.color * 0.1;
    let result = (ambient + in.vertex_light) * texture_color.xyz;

    return vec4<f32>(result, texture_color.a);
    //return textureSample(t_diffuse, s_diffuse, in.uv);
}
