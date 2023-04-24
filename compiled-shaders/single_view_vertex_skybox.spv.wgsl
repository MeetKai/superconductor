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
var<private> global_1: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 1.0);
var<private> global_2: i32;
var<private> global_3: vec3<f32>;

fn function() {
    let _e20 = global_2;
    let _e23 = fma(f32((_e20 / 2)), 4.0, -1.0);
    let _e26 = fma(f32((_e20 & 1)), 4.0, -1.0);
    let _e29 = global.member.member_12;
    let _e34 = f32((1u - select(0u, 1u, ((_e29 & 8u) == 8u))));
    let _e38 = global.member.member_2;
    let _e64 = (fma(_e38.member_2.x, _e34, fma(_e38.member.x, _e23, (_e38.member_1.x * _e26))) + _e38.member_3.x);
    let _e65 = (fma(_e38.member_2.y, _e34, fma(_e38.member.y, _e23, (_e38.member_1.y * _e26))) + _e38.member_3.y);
    let _e66 = (fma(_e38.member_2.z, _e34, fma(_e38.member.z, _e23, (_e38.member_1.z * _e26))) + _e38.member_3.z);
    let _e69 = global.member.member_4;
    let _e78 = fma(_e69.w, _e69.w, -(fma(_e69.z, _e69.z, fma(_e69.x, _e69.x, (_e69.y * _e69.y)))));
    let _e82 = (fma(_e66, _e69.z, fma(_e64, _e69.x, (_e65 * _e69.y))) * 2.0);
    let _e98 = (_e69.w * 2.0);
    global_3 = vec3<f32>(fma(fma(_e69.y, _e66, -((_e65 * _e69.z))), _e98, fma(_e64, _e78, (_e69.x * _e82))), fma(fma(_e69.z, _e64, -((_e66 * _e69.x))), _e98, fma(_e65, _e78, (_e69.y * _e82))), fma(fma(_e69.x, _e65, -((_e64 * _e69.y))), _e98, fma(_e66, _e78, (_e69.z * _e82))));
    global_1 = vec4<f32>(_e23, _e26, _e34, 1.0);
    if ((_e29 & 1u) == 1u) {
        let _e106 = global_1[1u];
        global_1[1u] = -(_e106);
    }
    return;
}

@vertex 
fn single_view__vertex_skybox(@builtin(vertex_index) param: u32) -> VertexOutput {
    global_2 = i32(param);
    function();
    let _e6 = global_1.y;
    global_1.y = -(_e6);
    let _e8 = global_1;
    let _e9 = global_3;
    return VertexOutput(_e8, _e9);
}
