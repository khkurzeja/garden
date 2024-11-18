// Idea and implementation based on https://poniesandlight.co.uk/reflect/debug_print_text/


//**TODO: I want to try not having vertex input?
//        Instead, just use instance data.
//        Is this possible?
//        Let's get things working with the vert buffer first.
struct VertexInput {
    @builtin(vertex_index) vert_id: u32,
    @location(0) base_position: vec2<f32>,
}

struct InstanceInput {
    @location(1) position: vec2<f32>,
    @location(2) scale: f32,
    @location(3) color: vec3<f32>,
    @location(4) char_id: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) char_id: u32,
    @location(2) coord: vec2f,
}

struct VertexUniforms {
    screen_to_clip: mat3x3<f32>,
};
@group(0) @binding(0)
var<uniform> vert_uniform: VertexUniforms;

struct FragmentUniforms {
    pixel_size: vec4f,
};
@group(0) @binding(1)
var<uniform> frag_uniform: FragmentUniforms;

@vertex
fn vs_main(in: VertexInput, instance: InstanceInput) -> VertexOutput {
    let p = vert_uniform.screen_to_clip * vec3<f32>(in.base_position * instance.scale + instance.position, 1.0);

    var out: VertexOutput;
    out.clip_position = vec4<f32>(p.xy, 0.0, 1.0);
    out.color = instance.color;
    out.char_id = instance.char_id;

    var vert_uv = array<vec2f, 4>(
        vec2f(0.0, 0.0),
        vec2f(7.9999, 0.0),
        vec2f(7.9999, 15.9999),
        vec2f(0.0, 15.9999),
    );
    out.coord = vert_uv[in.vert_id];

    return out;
}

//https://stackoverflow.com/a/4275343
fn rand(co: vec2f) -> f32 {
    return fract(sin(dot(co, vec2f(12.9898, 78.233))) * 43758.5453);
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    //**TODO: We could pass an entire char (vec4u) in per instance, to draw an arbitrary 8x16 binary bitmap.
    //        And this data can be stored on the rust side.
    var font_data = array<vec4u, 96>(
        vec4u( 0x00000000, 0x00000000, 0x00000000, 0x00000000 ),
        vec4u( 0x00001010, 0x10101010, 0x00001010, 0x00000000 ),
        vec4u( 0x00242424, 0x24000000, 0x00000000, 0x00000000 ),
        vec4u( 0x00000024, 0x247E2424, 0x247E2424, 0x00000000 ),
        vec4u( 0x00000808, 0x1E20201C, 0x02023C08, 0x08000000 ),
        vec4u( 0x00000030, 0x494A3408, 0x16294906, 0x00000000 ),
        vec4u( 0x00003048, 0x48483031, 0x49464639, 0x00000000 ),
        vec4u( 0x00101010, 0x10000000, 0x00000000, 0x00000000 ),
        vec4u( 0x00000408, 0x08101010, 0x10101008, 0x08040000 ),
        vec4u( 0x00002010, 0x10080808, 0x08080810, 0x10200000 ),
        vec4u( 0x00000000, 0x0024187E, 0x18240000, 0x00000000 ),
        vec4u( 0x00000000, 0x0808087F, 0x08080800, 0x00000000 ),
        vec4u( 0x00000000, 0x00000000, 0x00001818, 0x08081000 ),
        vec4u( 0x00000000, 0x0000007E, 0x00000000, 0x00000000 ),
        vec4u( 0x00000000, 0x00000000, 0x00001818, 0x00000000 ),
        vec4u( 0x00000202, 0x04040808, 0x10102020, 0x40400000 ),
        vec4u( 0x0000003C, 0x42464A52, 0x6242423C, 0x00000000 ),
        vec4u( 0x00000008, 0x18280808, 0x0808083E, 0x00000000 ),
        vec4u( 0x0000003C, 0x42020204, 0x0810207E, 0x00000000 ),
        vec4u( 0x0000007E, 0x04081C02, 0x0202423C, 0x00000000 ),
        vec4u( 0x00000004, 0x0C142444, 0x7E040404, 0x00000000 ),
        vec4u( 0x0000007E, 0x40407C02, 0x0202423C, 0x00000000 ),
        vec4u( 0x0000001C, 0x2040407C, 0x4242423C, 0x00000000 ),
        vec4u( 0x0000007E, 0x02040408, 0x08101010, 0x00000000 ),
        vec4u( 0x0000003C, 0x4242423C, 0x4242423C, 0x00000000 ),
        vec4u( 0x0000003C, 0x4242423E, 0x02020438, 0x00000000 ),
        vec4u( 0x00000000, 0x00181800, 0x00001818, 0x00000000 ),
        vec4u( 0x00000000, 0x00181800, 0x00001818, 0x08081000 ),
        vec4u( 0x00000004, 0x08102040, 0x20100804, 0x00000000 ),
        vec4u( 0x00000000, 0x00007E00, 0x007E0000, 0x00000000 ),
        vec4u( 0x00000020, 0x10080402, 0x04081020, 0x00000000 ),
        vec4u( 0x00003C42, 0x02040810, 0x00001010, 0x00000000 ),
        vec4u( 0x00001C22, 0x414F5151, 0x51534D40, 0x201F0000 ),
        vec4u( 0x00000018, 0x24424242, 0x7E424242, 0x00000000 ),
        vec4u( 0x0000007C, 0x4242427C, 0x4242427C, 0x00000000 ),
        vec4u( 0x0000001E, 0x20404040, 0x4040201E, 0x00000000 ),
        vec4u( 0x00000078, 0x44424242, 0x42424478, 0x00000000 ),
        vec4u( 0x0000007E, 0x4040407C, 0x4040407E, 0x00000000 ),
        vec4u( 0x0000007E, 0x4040407C, 0x40404040, 0x00000000 ),
        vec4u( 0x0000001E, 0x20404046, 0x4242221E, 0x00000000 ),
        vec4u( 0x00000042, 0x4242427E, 0x42424242, 0x00000000 ),
        vec4u( 0x0000003E, 0x08080808, 0x0808083E, 0x00000000 ),
        vec4u( 0x00000002, 0x02020202, 0x0242423C, 0x00000000 ),
        vec4u( 0x00000042, 0x44485060, 0x50484442, 0x00000000 ),
        vec4u( 0x00000040, 0x40404040, 0x4040407E, 0x00000000 ),
        vec4u( 0x00000041, 0x63554949, 0x41414141, 0x00000000 ),
        vec4u( 0x00000042, 0x62524A46, 0x42424242, 0x00000000 ),
        vec4u( 0x0000003C, 0x42424242, 0x4242423C, 0x00000000 ),
        vec4u( 0x0000007C, 0x4242427C, 0x40404040, 0x00000000 ),
        vec4u( 0x0000003C, 0x42424242, 0x4242423C, 0x04020000 ),
        vec4u( 0x0000007C, 0x4242427C, 0x48444242, 0x00000000 ),
        vec4u( 0x0000003E, 0x40402018, 0x0402027C, 0x00000000 ),
        vec4u( 0x0000007F, 0x08080808, 0x08080808, 0x00000000 ),
        vec4u( 0x00000042, 0x42424242, 0x4242423C, 0x00000000 ),
        vec4u( 0x00000042, 0x42424242, 0x24241818, 0x00000000 ),
        vec4u( 0x00000041, 0x41414149, 0x49495563, 0x00000000 ),
        vec4u( 0x00000041, 0x41221408, 0x14224141, 0x00000000 ),
        vec4u( 0x00000041, 0x41221408, 0x08080808, 0x00000000 ),
        vec4u( 0x0000007E, 0x04080810, 0x1020207E, 0x00000000 ),
        vec4u( 0x00001E10, 0x10101010, 0x10101010, 0x101E0000 ),
        vec4u( 0x00004040, 0x20201010, 0x08080404, 0x02020000 ),
        vec4u( 0x00007808, 0x08080808, 0x08080808, 0x08780000 ),
        vec4u( 0x00001028, 0x44000000, 0x00000000, 0x00000000 ),
        vec4u( 0x00000000, 0x00000000, 0x00000000, 0x00FF0000 ),
        vec4u( 0x00201008, 0x04000000, 0x00000000, 0x00000000 ),
        vec4u( 0x00000000, 0x003C0202, 0x3E42423E, 0x00000000 ),
        vec4u( 0x00004040, 0x407C4242, 0x4242427C, 0x00000000 ),
        vec4u( 0x00000000, 0x003C4240, 0x4040423C, 0x00000000 ),
        vec4u( 0x00000202, 0x023E4242, 0x4242423E, 0x00000000 ),
        vec4u( 0x00000000, 0x003C4242, 0x7E40403E, 0x00000000 ),
        vec4u( 0x00000E10, 0x107E1010, 0x10101010, 0x00000000 ),
        vec4u( 0x00000000, 0x003E4242, 0x4242423E, 0x02023C00 ),
        vec4u( 0x00004040, 0x407C4242, 0x42424242, 0x00000000 ),
        vec4u( 0x00000808, 0x00380808, 0x0808083E, 0x00000000 ),
        vec4u( 0x00000404, 0x001C0404, 0x04040404, 0x04043800 ),
        vec4u( 0x00004040, 0x40444850, 0x70484442, 0x00000000 ),
        vec4u( 0x00003808, 0x08080808, 0x0808083E, 0x00000000 ),
        vec4u( 0x00000000, 0x00774949, 0x49494949, 0x00000000 ),
        vec4u( 0x00000000, 0x007C4242, 0x42424242, 0x00000000 ),
        vec4u( 0x00000000, 0x003C4242, 0x4242423C, 0x00000000 ),
        vec4u( 0x00000000, 0x007C4242, 0x4242427C, 0x40404000 ),
        vec4u( 0x00000000, 0x003E4242, 0x4242423E, 0x02020200 ),
        vec4u( 0x00000000, 0x002E3020, 0x20202020, 0x00000000 ),
        vec4u( 0x00000000, 0x003E4020, 0x1804027C, 0x00000000 ),
        vec4u( 0x00000010, 0x107E1010, 0x1010100E, 0x00000000 ),
        vec4u( 0x00000000, 0x00424242, 0x4242423E, 0x00000000 ),
        vec4u( 0x00000000, 0x00424242, 0x24241818, 0x00000000 ),
        vec4u( 0x00000000, 0x00414141, 0x49495563, 0x00000000 ),
        vec4u( 0x00000000, 0x00412214, 0x08142241, 0x00000000 ),
        vec4u( 0x00000000, 0x00424242, 0x4242423E, 0x02023C00 ),
        vec4u( 0x00000000, 0x007E0408, 0x1020407E, 0x00000000 ),
        vec4u( 0x000E1010, 0x101010E0, 0x10101010, 0x100E0000 ),
        vec4u( 0x00080808, 0x08080808, 0x08080808, 0x08080000 ),
        vec4u( 0x00700808, 0x08080807, 0x08080808, 0x08700000 ),
        vec4u( 0x00003149, 0x46000000, 0x00000000, 0x00000000 ),
        vec4u( 0x00000000, 0x00000000, 0x00000000, 0x00000000 ),
    );

    let char_code = in.char_id - 0x20;  // lowest char is space, 0x20 hex

    if char_code == 0x00 {
        discard;
    }

    // Basic antialiasing
    var value = 0.0;
    for (var j = -1; j <= 1; j++) {
        for (var i = -1; i <= 1; i++) {
            let pixel = vec2u(in.coord + vec2f(f32(i), f32(j)) * frag_uniform.pixel_size.xy/4.0);

            let four_lines = font_data[char_code][pixel.y / 4];

            // Now we must pick the correct line
            // Note 3- ... this is because big/little endian
            let current_line  = (four_lines >> (8u * (3u - pixel.y % 4u))) & 0xffu;
            let pixel_value = (current_line >> (7u - pixel.x)) & 0x01u;

            value += f32(pixel_value) / 9.0;
        }
    }
    //value /= 9.0;

    return vec4f(mix(vec3f(1.0), in.color, value), 1.0);


    // Jitter antialiasing
    // var value = 0.0;
    // for (var j = -5; j <= 5; j++) {
    //     for (var i = -5; i <= 5; i++) {
    //         let jitter_x = rand(vec2f(f32(i), f32(j))) * 2.0 - 1.0;
    //         let jitter_y = rand(vec2f(f32(i) + 300.0, f32(j) + 700.0)) * 2.0 - 1.0;
    //         let jitter = vec2f(jitter_x, jitter_y) * frag_uniform.pixel_size.xy;
    //         let pixel = vec2u(in.coord + jitter);

    //         let four_lines = font_data[char_code][pixel.y / 4];

    //         // Now we must pick the correct line
    //         // Note 3- ... this is because big/little endian
    //         let current_line  = (four_lines >> (8u * (3u - pixel.y % 4u))) & 0xffu;
    //         let pixel_value = (current_line >> (7u - pixel.x)) & 0x01u;

    //         value += f32(pixel_value) / 100.0;
    //     }
    // }

    // return vec4f(mix(vec3f(1.0), in.color, value), 1.0);


    // Subpixel antialiasing
    // var value = vec3f(0.0);
    // for (var j = -1; j <= 1; j++) {
    //     for (var i = -1; i <= 1; i++) {
    //         let pixel = vec2u(in.coord + vec2f(f32(i), f32(j)) * frag_uniform.pixel_size.xy/4.0);

    //         let four_lines = font_data[char_code][pixel.y / 4];

    //         // Now we must pick the correct line
    //         // Note 3- ... this is because big/little endian
    //         let current_line  = (four_lines >> (8u * (3u - pixel.y % 4u))) & 0xffu;
    //         let pixel_value = (current_line >> (7u - pixel.x)) & 0x01u;

    //         value[i+1] += f32(pixel_value);
    //     }
    // }
    // value /= 3.0;

    // return vec4f(mix(vec3f(1.0), in.color, value), 1.0);
}
