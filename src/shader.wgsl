// Vertex shader

@group(1) @binding(0) // 1.
var<uniform> time: f32;


struct CameraUniform {
    view_proj: mat4x4<f32>,
}

@group(2) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) normal: vec3<f32>,
    @builtin(vertex_index) vertex_index: u32,
    @builtin(instance_index) instance_index: u32,
}

struct InstanceInput {
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) normal: vec3<f32>,
}

@vertex
fn vs_main( model: VertexInput, instance: InstanceInput ) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

    var out: VertexOutput;

    out.clip_position = camera.view_proj * model_matrix * vec4<f32>(model.position.xyz, 1.0);

    out.tex_coords = model.tex_coords;
    out.normal = model.normal;

    return out;
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0)@binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let light_str = dot(in.normal, normalize(vec3<f32>(0.0, 1.0, 1.0))) + 1.0;
    return textureSample(t_diffuse, s_diffuse, in.tex_coords)*light_str;
}