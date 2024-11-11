struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
}


@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(in.position, 1.0);
    out.color = in.color;
    return out;
}


// Fragment shader

struct UniformData {
    center: vec2<f32>,
    radius: f32,
};
@group(0) @binding(0)
var<uniform> uni: UniformData;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let d = distance(uni.center, in.clip_position.xy) - uni.radius;
    let off = vec4(.0, .0, .0, 1.0);
    let on = vec4(in.color, 1.0);
    return select(off, on, d <= 0.0);
}
