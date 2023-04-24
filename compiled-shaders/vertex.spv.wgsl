struct type_11 {
    member: vec4<f32>,
    member_1: vec4<f32>,
    member_2: vec4<f32>,
    member_3: vec4<f32>,
}

struct type_12 {
    member: type_11,
    member_1: type_11,
    member_2: type_11,
    member_3: type_11,
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

struct type_13 {
    member: type_12,
}

struct type_15 {
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

struct type_17 {
    member: type_15,
}

struct VertexOutput {
    @builtin(position) member: vec4<f32>,
    @location(0) member_1: vec3<f32>,
    @location(1) member_2: vec3<f32>,
    @location(2) member_3: vec2<f32>,
    @location(3) member_4: vec2<f32>,
    @location(4) member_5: u32,
    @location(5) member_6: u32,
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
var<uniform> global_8: type_13;
@group(1) @binding(4) 
var<uniform> global_9: type_17;
var<private> global_10: i32;
var<private> global_11: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 1.0);
var<private> global_12: vec3<f32>;
var<private> global_13: vec3<f32>;
var<private> global_14: vec2<f32>;
var<private> global_15: u32;
var<private> global_16: vec2<f32>;
var<private> global_17: u32;

fn function() {
    var phi_153_: type_11;

    let _e29 = global;
    let _e30 = global_1;
    let _e31 = global_2;
    let _e32 = global_3;
    let _e33 = global_4;
    let _e34 = global_5;
    let _e35 = global_6;
    let _e36 = global_7;
    let _e38 = global_10;
    let _e43 = (_e33 * _e29.w);
    let _e55 = fma(_e30.w, _e30.w, -(fma(_e30.z, _e30.z, fma(_e30.x, _e30.x, (_e30.y * _e30.y)))));
    let _e59 = (fma(_e43.z, _e30.z, fma(_e43.x, _e30.x, (_e43.y * _e30.y))) * 2.0);
    let _e75 = (_e30.w * 2.0);
    let _e79 = (_e29.x + fma(fma(_e30.y, _e43.z, -((_e43.y * _e30.z))), _e75, fma(_e43.x, _e55, (_e30.x * _e59))));
    let _e80 = (_e29.y + fma(fma(_e30.z, _e43.x, -((_e43.z * _e30.x))), _e75, fma(_e43.y, _e55, (_e30.y * _e59))));
    let _e81 = (_e29.z + fma(fma(_e30.x, _e43.y, -((_e43.x * _e30.y))), _e75, fma(_e43.z, _e55, (_e30.z * _e59))));
    if (_e38 == 0) {
        let _e86 = global_8.member.member;
        phi_153_ = _e86;
    } else {
        let _e89 = global_8.member.member_1;
        phi_153_ = _e89;
    }
    let _e91 = phi_153_;
    global_11 = vec4<f32>((fma(_e91.member_2.x, _e81, fma(_e91.member.x, _e79, (_e91.member_1.x * _e80))) + _e91.member_3.x), (fma(_e91.member_2.y, _e81, fma(_e91.member.y, _e79, (_e91.member_1.y * _e80))) + _e91.member_3.y), (fma(_e91.member_2.z, _e81, fma(_e91.member.z, _e79, (_e91.member_1.z * _e80))) + _e91.member_3.z), (fma(_e91.member_2.w, _e81, fma(_e91.member.w, _e79, (_e91.member_1.w * _e80))) + _e91.member_3.w));
    global_12 = vec3<f32>(_e79, _e80, _e81);
    let _e135 = (fma(_e34.z, _e30.z, fma(_e34.x, _e30.x, (_e34.y * _e30.y))) * 2.0);
    global_13 = vec3<f32>(fma(fma(_e30.y, _e34.z, -((_e34.y * _e30.z))), _e75, fma(_e34.x, _e55, (_e30.x * _e135))), fma(fma(_e30.z, _e34.x, -((_e34.z * _e30.x))), _e75, fma(_e34.y, _e55, (_e30.y * _e135))), fma(fma(_e30.x, _e34.y, -((_e34.x * _e30.y))), _e75, fma(_e34.z, _e55, (_e30.z * _e135))));
    let _e156 = global_9.member.member_1;
    let _e158 = global_9.member.member_6;
    let _e160 = global_9.member.member_2;
    let _e161 = sin(_e158);
    let _e162 = cos(_e158);
    global_14 = (_e156 + vec2<f32>((fma(_e162, _e160.x, (-(_e161) * _e160.y)) * _e35.x), (fma(_e161, _e160.x, (_e162 * _e160.y)) * _e35.y)));
    global_15 = _e31;
    global_16 = _e36;
    global_17 = _e32;
    let _e178 = global_8.member.member_12;
    if ((_e178 & 1u) == 1u) {
        let _e182 = global_11[1u];
        global_11[1u] = -(_e182);
    }
    return;
}

@vertex 
fn vertex(@location(0) param: vec4<f32>, @location(1) param_1: vec4<f32>, @location(3) param_2: u32, @location(4) param_3: u32, @location(5) param_4: vec3<f32>, @location(6) param_5: vec3<f32>, @location(7) param_6: vec2<f32>, @location(8) param_7: vec2<f32>, @builtin(view_index) param_8: i32) -> VertexOutput {
    global = param;
    global_1 = param_1;
    global_2 = param_2;
    global_3 = param_3;
    global_4 = param_4;
    global_5 = param_5;
    global_6 = param_6;
    global_7 = param_7;
    global_10 = param_8;
    function();
    let _e26 = global_11.y;
    global_11.y = -(_e26);
    let _e28 = global_11;
    let _e29 = global_12;
    let _e30 = global_13;
    let _e31 = global_14;
    let _e32 = global_16;
    let _e33 = global_15;
    let _e34 = global_17;
    return VertexOutput(_e28, _e29, _e30, _e31, _e32, _e33, _e34);
}
