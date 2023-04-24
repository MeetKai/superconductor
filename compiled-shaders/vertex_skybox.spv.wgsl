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

struct VertexOutput {
    @builtin(position) member: vec4<f32>,
    @location(0) member_1: vec3<f32>,
}

@group(0) @binding(0) 
var<uniform> global: type_10;
var<private> global_1: i32;
var<private> global_2: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 1.0);
var<private> global_3: i32;
var<private> global_4: vec3<f32>;

fn function() {
    var phi_92_: type_8;
    var phi_139_: vec4<f32>;

    let _e24 = global_3;
    let _e25 = global_1;
    let _e28 = fma(f32((_e24 / 2)), 4.0, -1.0);
    let _e31 = fma(f32((_e24 & 1)), 4.0, -1.0);
    let _e34 = global.member.member_12;
    let _e39 = f32((1u - select(0u, 1u, ((_e34 & 8u) == 8u))));
    let _e41 = (_e25 == 0);
    if _e41 {
        let _e44 = global.member.member_2;
        phi_92_ = _e44;
    } else {
        let _e47 = global.member.member_3;
        phi_92_ = _e47;
    }
    let _e49 = phi_92_;
    let _e75 = (fma(_e49.member_2.x, _e39, fma(_e49.member.x, _e28, (_e49.member_1.x * _e31))) + _e49.member_3.x);
    let _e76 = (fma(_e49.member_2.y, _e39, fma(_e49.member.y, _e28, (_e49.member_1.y * _e31))) + _e49.member_3.y);
    let _e77 = (fma(_e49.member_2.z, _e39, fma(_e49.member.z, _e28, (_e49.member_1.z * _e31))) + _e49.member_3.z);
    if _e41 {
        let _e80 = global.member.member_4;
        phi_139_ = _e80;
    } else {
        let _e83 = global.member.member_5;
        phi_139_ = _e83;
    }
    let _e85 = phi_139_;
    let _e94 = fma(_e85.w, _e85.w, -(fma(_e85.z, _e85.z, fma(_e85.x, _e85.x, (_e85.y * _e85.y)))));
    let _e98 = (fma(_e77, _e85.z, fma(_e75, _e85.x, (_e76 * _e85.y))) * 2.0);
    let _e114 = (_e85.w * 2.0);
    global_4 = vec3<f32>(fma(fma(_e85.y, _e77, -((_e76 * _e85.z))), _e114, fma(_e75, _e94, (_e85.x * _e98))), fma(fma(_e85.z, _e75, -((_e77 * _e85.x))), _e114, fma(_e76, _e94, (_e85.y * _e98))), fma(fma(_e85.x, _e76, -((_e75 * _e85.y))), _e114, fma(_e77, _e94, (_e85.z * _e98))));
    global_2 = vec4<f32>(_e28, _e31, _e39, 1.0);
    if ((_e34 & 1u) == 1u) {
        let _e122 = global_2[1u];
        global_2[1u] = -(_e122);
    }
    return;
}

@vertex 
fn vertex_skybox(@builtin(vertex_index) param: u32, @builtin(view_index) param_1: i32) -> VertexOutput {
    global_3 = i32(param);
    global_1 = param_1;
    function();
    let _e8 = global_2.y;
    global_2.y = -(_e8);
    let _e10 = global_2;
    let _e11 = global_4;
    return VertexOutput(_e10, _e11);
}
