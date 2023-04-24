struct type_6 {
    member: vec4<f32>,
    member_1: vec4<f32>,
    member_2: vec4<f32>,
    member_3: vec4<f32>,
}

struct type_7 {
    member: type_6,
    member_1: type_6,
    member_2: type_6,
    member_3: type_6,
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

struct type_8 {
    member: type_7,
}

@group(0) @binding(0) 
var<uniform> global: type_8;
var<private> global_1: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 1.0);
var<private> global_2: vec3<f32>;
var<private> global_3: vec4<f32>;
var<private> global_4: vec4<f32>;

fn function() {
    let _e13 = global_2;
    let _e14 = global_3;
    let _e15 = global_4;
    let _e20 = (_e13 * _e14.w);
    let _e32 = fma(_e15.w, _e15.w, -(fma(_e15.z, _e15.z, fma(_e15.x, _e15.x, (_e15.y * _e15.y)))));
    let _e36 = (fma(_e20.z, _e15.z, fma(_e20.x, _e15.x, (_e20.y * _e15.y))) * 2.0);
    let _e52 = (_e15.w * 2.0);
    let _e56 = (_e14.x + fma(fma(_e15.y, _e20.z, -((_e20.y * _e15.z))), _e52, fma(_e20.x, _e32, (_e15.x * _e36))));
    let _e57 = (_e14.y + fma(fma(_e15.z, _e20.x, -((_e20.z * _e15.x))), _e52, fma(_e20.y, _e32, (_e15.y * _e36))));
    let _e58 = (_e14.z + fma(fma(_e15.x, _e20.y, -((_e20.x * _e15.y))), _e52, fma(_e20.z, _e32, (_e15.z * _e36))));
    let _e61 = global.member.member;
    global_1 = vec4<f32>((fma(_e61.member_2.x, _e58, fma(_e61.member.x, _e56, (_e61.member_1.x * _e57))) + _e61.member_3.x), (fma(_e61.member_2.y, _e58, fma(_e61.member.y, _e56, (_e61.member_1.y * _e57))) + _e61.member_3.y), (fma(_e61.member_2.z, _e58, fma(_e61.member.z, _e56, (_e61.member_1.z * _e57))) + _e61.member_3.z), (fma(_e61.member_2.w, _e58, fma(_e61.member.w, _e56, (_e61.member_1.w * _e57))) + _e61.member_3.w));
    let _e101 = global.member.member_12;
    if ((_e101 & 1u) == 1u) {
        let _e105 = global_1[1u];
        global_1[1u] = -(_e105);
    }
    return;
}

@vertex 
fn single_view__depth_prepass_vertex(@location(0) param: vec3<f32>, @location(1) param_1: vec4<f32>, @location(2) param_2: vec4<f32>) -> @builtin(position) vec4<f32> {
    global_2 = param;
    global_3 = param_1;
    global_4 = param_2;
    function();
    let _e8 = global_1.y;
    global_1.y = -(_e8);
    let _e10 = global_1;
    return _e10;
}
