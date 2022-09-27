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
    @builtin(vertex_index) vertex_index: u32
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main( model: VertexInput ) -> VertexOutput {
    var out: VertexOutput;
    var wobble: f32 = sin(time*10.0+f32(model.vertex_index)) * 0.05;
    var sidewobble: f32 = sin(time*10.0+f32(model.vertex_index)) * 0.05;
    var frontwobble: f32 = sin(time*10.0+f32(model.vertex_index)) * 0.05;
    if (i32(model.vertex_index) > 0) {wobble = wobble * -1.0;}
    if (model.position.y > 0.0) {sidewobble = 0.0;}
    out.tex_coords = model.tex_coords;
    out.clip_position = camera.view_proj * vec4<f32>(model.position.x+sidewobble, model.position.y+wobble, model.position.z+frontwobble, 1.0);
    return out;
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0)@binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}