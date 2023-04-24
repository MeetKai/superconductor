struct type_9 {
    member: vec4<f32>,
    member_1: vec4<f32>,
    member_2: vec4<f32>,
    member_3: vec4<f32>,
}

struct type_10 {
    member: type_9,
    member_1: type_9,
    member_2: type_9,
    member_3: type_9,
    member_4: vec4<f32>,
    member_5: vec4<f32>,
    member_6: f32,
    member_7: f32,
    member_8: f32,
    member_9: f32,
    member_10: f32,
    member_11: f32,
    member_12: u32,
    member_13: f32,
    member_14: f32,
    member_15: f32,
    member_16: f32,
    member_17: f32,
    member_18: f32,
    member_19: f32,
}

struct type_11 {
    member: type_10,
}

struct type_13 {
    member: vec4<f32>,
    member_1: vec2<f32>,
    member_2: vec2<f32>,
    member_3: f32,
    member_4: f32,
    member_5: f32,
    member_6: f32,
    member_7: f32,
    member_8: f32,
    member_9: f32,
    member_10: u32,
}

struct type_15 {
    member: type_13,
}

struct VertexOutput {
    @builtin(position) member: vec4<f32>,
    @location(0) member_1: vec3<f32>,
    @location(1) member_2: vec3<f32>,
    @location(2) member_3: vec2<f32>,
    @location(3) member_4: vec2<f32>,
    @location(4) @interpolate(flat) member_5: u32,
    @location(5) @interpolate(flat) member_6: u32,
}

var<private> global: vec4<f32>;
var<private> global_1: vec4<f32>;
var<private> global_2: u32;
var<private> global_3: u32;
var<private> global_4: vec3<f32>;
var<private> global_5: vec3<f32>;
var<private> global_6: vec2<f32>;
var<private> global_7: vec2<f32>;
@group(0) @binding(0) 
var<uniform> global_8: type_11;
@group(1) @binding(4) 
var<uniform> global_9: type_15;
var<private> global_10: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 1.0);
var<private> global_11: vec3<f32>;
var<private> global_12: vec3<f32>;
var<private> global_13: vec2<f32>;
var<private> global_14: u32;
var<private> global_15: vec2<f32>;
var<private> global_16: u32;

fn function() {
    let _e27 = global;
    let _e28 = global_1;
    let _e29 = global_2;
    let _e30 = global_3;
    let _e31 = global_4;
    let _e32 = global_5;
    let _e33 = global_6;
    let _e34 = global_7;
    let _e40 = (_e31 * _e27.w);
    let _e52 = fma(_e28.w, _e28.w, -(fma(_e28.z, _e28.z, fma(_e28.x, _e28.x, (_e28.y * _e28.y)))));
    let _e56 = (fma(_e40.z, _e28.z, fma(_e40.x, _e28.x, (_e40.y * _e28.y))) * 2.0);
    let _e72 = (_e28.w * 2.0);
    let _e76 = (_e27.x + fma(fma(_e28.y, _e40.z, -((_e40.y * _e28.z))), _e72, fma(_e40.x, _e52, (_e28.x * _e56))));
    let _e77 = (_e27.y + fma(fma(_e28.z, _e40.x, -((_e40.z * _e28.x))), _e72, fma(_e40.y, _e52, (_e28.y * _e56))));
    let _e78 = (_e27.z + fma(fma(_e28.x, _e40.y, -((_e40.x * _e28.y))), _e72, fma(_e40.z, _e52, (_e28.z * _e56))));
    let _e82 = global_8.member.member;
    global_10 = vec4<f32>((fma(_e82.member_2.x, _e78, fma(_e82.member.x, _e76, (_e82.member_1.x * _e77))) + _e82.member_3.x), (fma(_e82.member_2.y, _e78, fma(_e82.member.y, _e76, (_e82.member_1.y * _e77))) + _e82.member_3.y), (fma(_e82.member_2.z, _e78, fma(_e82.member.z, _e76, (_e82.member_1.z * _e77))) + _e82.member_3.z), (fma(_e82.member_2.w, _e78, fma(_e82.member.w, _e76, (_e82.member_1.w * _e77))) + _e82.member_3.w));
    global_11 = vec3<f32>(_e76, _e77, _e78);
    let _e126 = (fma(_e32.z, _e28.z, fma(_e32.x, _e28.x, (_e32.y * _e28.y))) * 2.0);
    global_12 = vec3<f32>(fma(fma(_e28.y, _e32.z, -((_e32.y * _e28.z))), _e72, fma(_e32.x, _e52, (_e28.x * _e126))), fma(fma(_e28.z, _e32.x, -((_e32.z * _e28.x))), _e72, fma(_e32.y, _e52, (_e28.y * _e126))), fma(fma(_e28.x, _e32.y, -((_e32.x * _e28.y))), _e72, fma(_e32.z, _e52, (_e28.z * _e126))));
    let _e147 = global_9.member.member_1;
    let _e149 = global_9.member.member_6;
    let _e151 = global_9.member.member_2;
    let _e152 = sin(_e149);
    let _e153 = cos(_e149);
    global_13 = (_e147 + vec2<f32>((fma(_e153, _e151.x, (-(_e152) * _e151.y)) * _e33.x), (fma(_e152, _e151.x, (_e153 * _e151.y)) * _e33.y)));
    global_14 = _e29;
    global_15 = _e34;
    global_16 = _e30;
    let _e169 = global_8.member.member_12;
    if ((_e169 & 1u) == 1u) {
        let _e173 = global_10[1u];
        global_10[1u] = -(_e173);
    }
    return;
}

@vertex 
fn single_view__vertex(@location(0) param: vec4<f32>, @location(1) param_1: vec4<f32>, @location(3) param_2: u32, @location(4) param_3: u32, @location(5) param_4: vec3<f32>, @location(6) param_5: vec3<f32>, @location(7) param_6: vec2<f32>, @location(8) param_7: vec2<f32>) -> VertexOutput {
    global = param;
    global_1 = param_1;
    global_2 = param_2;
    global_3 = param_3;
    global_4 = param_4;
    global_5 = param_5;
    global_6 = param_6;
    global_7 = param_7;
    function();
    let _e24 = global_10.y;
    global_10.y = -(_e24);
    let _e26 = global_10;
    let _e27 = global_11;
    let _e28 = global_12;
    let _e29 = global_13;
    let _e30 = global_15;
    let _e31 = global_14;
    let _e32 = global_16;
    return VertexOutput(_e26, _e27, _e28, _e29, _e30, _e31, _e32);
}
