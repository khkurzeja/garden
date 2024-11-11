struct VertexInput {
    @location(0) position: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
}


@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(in.position, 1.0);
    return out;
}


//====================================================================================================

struct UniformData {
    camera: mat3x3<f32>,
    map_camera: mat3x3<f32>,
    screen_to_clip: mat3x3<f32>,
};
@group(0) @binding(0)
var<uniform> uf: UniformData;

fn sq(x:f32) -> f32 { return x*x; }
fn sq2(x: vec2f) -> f32 { return dot(x,x); }
fn sq3(x: vec3f) -> f32 { return dot(x,x); }

fn floor_mod(x: f32) -> f32 { return x - floor(x); }
fn floor_mod_by(x: f32, y: f32) -> f32 { return floor_mod(x / y) * y; }

fn floor_mod2(x: vec2f) -> vec2f { return x - floor(x); }
fn floor_mod_by2(x: vec2f, y: vec2f) -> vec2f { return floor_mod2(x / y) * y; }

fn mat3_to_mat2(a: mat3x3<f32>) -> mat2x2<f32> {
    return mat2x2<f32>(a[0].xy, a[1].xy);
}

// Map and shader inspiration and implementation from 
// https://www.shadertoy.com/view/Wd2cRG
fn map(q: vec2f) -> vec3f {
    return vec3f(q.xy, 1.0 - sq2(q));
}

fn map_jac(q: vec2f) -> mat2x3<f32> {
    let column1 = vec3f(1.0, 0.0, -2.0 * q.x);
	let column2 = vec3f(0.0, 1.0, -2.0 * q.y);
	return mat2x3<f32>(column1, column2);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> 
{
    var q = (uf.screen_to_clip * vec3f(in.clip_position.xy, 1.0)).xy;
    q = (uf.map_camera * vec3f(q, 1.0)).xy;

    let d_bg = 1e38;  // inf
    let bg_color = vec3f(1.0, 1.0, 1.0);

    let jac = uf.camera * map_jac(q) * mat3_to_mat2(uf.map_camera) * mat3_to_mat2(uf.screen_to_clip);

    var Q = map(q);
    Q = uf.camera * Q;
    

    var d_grid: f32;
    let grid_color = vec3f(.6, .6, .6);
    { // Grid lines
        let mq = floor_mod2(Q.xy/Q.z + .5);
        let d1 = ((abs(mq.x - .5) * 2.0) / length(vec3f(1.0/Q.z, .0, -Q.x/sq(Q.z)) * jac)) - 1.0;
        let d2 = ((abs(mq.y - .5) * 2.0) / length(vec3f(.0, 1.0/Q.z, -Q.y/sq(Q.z)) * jac)) - 1.0;
        d_grid = min(d1, d2);
    }

    var d_x_axis: f32;
    let x_axis_color = vec3f(.7, .4, .4);
    var d_y_axis: f32;
    let y_axis_color = vec3f(.4, .8, .4);
    { // Axis lines
        let mq = Q.xy/Q.z + .5;
        d_y_axis = ((abs(mq.x - .5) * 2.0) / length(vec3f(1.0/Q.z, .0, -Q.x/sq(Q.z)) * jac)) - 1.5;
        d_x_axis = ((abs(mq.y - .5) * 2.0) / length(vec3f(.0, 1.0/Q.z, -Q.y/sq(Q.z)) * jac)) - 1.5;
    }

    let falloff = 1.0;

    var out_color = bg_color;
	out_color = mix(grid_color, out_color, smoothstep(.0, falloff, d_grid));
    out_color = mix(x_axis_color, out_color, smoothstep(.0, falloff, d_x_axis));
    out_color = mix(y_axis_color, out_color, smoothstep(.0, falloff, d_y_axis));

    return vec4f(out_color, 1.0);
}
