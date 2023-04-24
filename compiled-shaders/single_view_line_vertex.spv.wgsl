struct type_7 {
    member: vec4<f32>,
    member_1: vec4<f32>,
    member_2: vec4<f32>,
    member_3: vec4<f32>,
}

struct type_8 {
    member: type_7,
    member_1: type_7,
    member_2: type_7,
    member_3: type_7,
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

struct type_9 {
    member: type_8,
}

struct VertexOutput {
    @builtin(position) member: vec4<f32>,
    @location(0) member_1: vec3<f32>,
}

@group(0) @binding(0) 
var<uniform> global: type_9;
var<private> global_1: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 1.0);
var<private> global_2: vec3<f32>;
var<private> global_3: u32;
var<private> global_4: vec3<f32>;

fn function() {
    var local: array<vec3<f32>,16u> = array<vec3<f32>,16u>(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.1647000014781952), vec3<f32>(0.0, 0.0, 0.36469998955726624), vec3<f32>(0.0, 0.0, 0.6646999716758728), vec3<f32>(0.0, 0.0, 0.9646999835968018), vec3<f32>(0.0, 0.9254999756813049, 0.9254999756813049), vec3<f32>(0.0, 0.5647000074386597, 0.0), vec3<f32>(0.0, 0.7843000292778015, 0.0), vec3<f32>(1.0, 1.0, 0.0), vec3<f32>(0.9058799743652344, 0.7529399991035461, 0.0), vec3<f32>(1.0, 0.5647000074386597, 0.0), vec3<f32>(1.0, 0.0, 0.0), vec3<f32>(0.8392000198364258, 0.0, 0.0), vec3<f32>(1.0, 0.0, 1.0), vec3<f32>(0.6000000238418579, 0.33329999446868896, 0.7882000207901001), vec3<f32>(1.0, 1.0, 1.0));

    let _e46 = global_2;
    let _e47 = global_3;
    let _e50 = global.member.member;
    global_1 = vec4<f32>((fma(_e50.member_2.x, _e46.z, fma(_e50.member.x, _e46.x, (_e50.member_1.x * _e46.y))) + _e50.member_3.x), (fma(_e50.member_2.y, _e46.z, fma(_e50.member.y, _e46.x, (_e50.member_1.y * _e46.y))) + _e50.member_3.y), (fma(_e50.member_2.z, _e46.z, fma(_e50.member.z, _e46.x, (_e50.member_1.z * _e46.y))) + _e50.member_3.z), (fma(_e50.member_2.w, _e46.z, fma(_e50.member.w, _e46.x, (_e50.member_1.w * _e46.y))) + _e50.member_3.w));
    let _e93 = local[(_e47 % 16u)];
    global_4 = _e93;
    let _e96 = global.member.member_12;
    if ((_e96 & 1u) == 1u) {
        let _e100 = global_1[1u];
        global_1[1u] = -(_e100);
    }
    return;
}

@vertex 
fn single_view__line_vertex(@location(0) param: vec3<f32>, @location(1) param_1: u32) -> VertexOutput {
    global_2 = param;
    global_3 = param_1;
    function();
    let _e7 = global_1.y;
    global_1.y = -(_e7);
    let _e9 = global_1;
    let _e10 = global_4;
    return VertexOutput(_e9, _e10);
}
