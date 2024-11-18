struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

struct VertexUniforms {
    screen_to_clip: mat3x3<f32>,
};
@group(0) @binding(0)
var<uniform> vert_uniform: VertexUniforms;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    let p = vert_uniform.screen_to_clip * vec3<f32>(in.position, 1.0);
    var out: VertexOutput;
    out.clip_position = vec4<f32>(p.xy, 0.0, 1.0);
    out.color = in.color;
    return out;
}


// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4(in.color, 1.0);
}
