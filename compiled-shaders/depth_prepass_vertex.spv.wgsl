struct type_8 {
    member: vec4<f32>,
    member_1: vec4<f32>,
    member_2: vec4<f32>,
    member_3: vec4<f32>,
}

struct type_9 {
    member: type_8,
    member_1: type_8,
    member_2: type_8,
    member_3: type_8,
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

struct type_10 {
    member: type_9,
}

@group(0) @binding(0) 
var<uniform> global: type_10;
var<private> global_1: i32;
var<private> global_2: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 1.0);
var<private> global_3: vec3<f32>;
var<private> global_4: vec4<f32>;
var<private> global_5: vec4<f32>;

fn function() {
    var phi_127_: type_8;

    let _e15 = global_3;
    let _e16 = global_4;
    let _e17 = global_5;
    let _e18 = global_1;
    let _e23 = (_e15 * _e16.w);
    let _e35 = fma(_e17.w, _e17.w, -(fma(_e17.z, _e17.z, fma(_e17.x, _e17.x, (_e17.y * _e17.y)))));
    let _e39 = (fma(_e23.z, _e17.z, fma(_e23.x, _e17.x, (_e23.y * _e17.y))) * 2.0);
    let _e55 = (_e17.w * 2.0);
    let _e59 = (_e16.x + fma(fma(_e17.y, _e23.z, -((_e23.y * _e17.z))), _e55, fma(_e23.x, _e35, (_e17.x * _e39))));
    let _e60 = (_e16.y + fma(fma(_e17.z, _e23.x, -((_e23.z * _e17.x))), _e55, fma(_e23.y, _e35, (_e17.y * _e39))));
    let _e61 = (_e16.z + fma(fma(_e17.x, _e23.y, -((_e23.x * _e17.y))), _e55, fma(_e23.z, _e35, (_e17.z * _e39))));
    if (_e18 == 0) {
        let _e65 = global.member.member;
        phi_127_ = _e65;
    } else {
        let _e68 = global.member.member_1;
        phi_127_ = _e68;
    }
    let _e70 = phi_127_;
    global_2 = vec4<f32>((fma(_e70.member_2.x, _e61, fma(_e70.member.x, _e59, (_e70.member_1.x * _e60))) + _e70.member_3.x), (fma(_e70.member_2.y, _e61, fma(_e70.member.y, _e59, (_e70.member_1.y * _e60))) + _e70.member_3.y), (fma(_e70.member_2.z, _e61, fma(_e70.member.z, _e59, (_e70.member_1.z * _e60))) + _e70.member_3.z), (fma(_e70.member_2.w, _e61, fma(_e70.member.w, _e59, (_e70.member_1.w * _e60))) + _e70.member_3.w));
    let _e110 = global.member.member_12;
    if ((_e110 & 1u) == 1u) {
        let _e114 = global_2[1u];
        global_2[1u] = -(_e114);
    }
    return;
}

@vertex 
fn depth_prepass_vertex(@location(0) param: vec3<f32>, @location(1) param_1: vec4<f32>, @location(2) param_2: vec4<f32>, @builtin(view_index) param_3: i32) -> @builtin(position) vec4<f32> {
    global_3 = param;
    global_4 = param_1;
    global_5 = param_2;
    global_1 = param_3;
    function();
    let _e10 = global_2.y;
    global_2.y = -(_e10);
    let _e12 = global_2;
    return _e12;
}
