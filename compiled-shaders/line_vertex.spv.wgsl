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

struct VertexOutput {
    @builtin(position) member: vec4<f32>,
    @location(0) member_1: vec3<f32>,
}

@group(0) @binding(0) 
var<uniform> global: type_11;
var<private> global_1: i32;
var<private> global_2: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 1.0);
var<private> global_3: vec3<f32>;
var<private> global_4: u32;
var<private> global_5: vec3<f32>;

fn function() {
    var local: array<vec3<f32>,16u> = array<vec3<f32>,16u>(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.1647000014781952), vec3<f32>(0.0, 0.0, 0.36469998955726624), vec3<f32>(0.0, 0.0, 0.6646999716758728), vec3<f32>(0.0, 0.0, 0.9646999835968018), vec3<f32>(0.0, 0.9254999756813049, 0.9254999756813049), vec3<f32>(0.0, 0.5647000074386597, 0.0), vec3<f32>(0.0, 0.7843000292778015, 0.0), vec3<f32>(1.0, 1.0, 0.0), vec3<f32>(0.9058799743652344, 0.7529399991035461, 0.0), vec3<f32>(1.0, 0.5647000074386597, 0.0), vec3<f32>(1.0, 0.0, 0.0), vec3<f32>(0.8392000198364258, 0.0, 0.0), vec3<f32>(1.0, 0.0, 1.0), vec3<f32>(0.6000000238418579, 0.33329999446868896, 0.7882000207901001), vec3<f32>(1.0, 1.0, 1.0));
    var phi_108_: type_9;

    let _e48 = global_3;
    let _e49 = global_4;
    let _e50 = global_1;
    if (_e50 == 0) {
        let _e54 = global.member.member;
        phi_108_ = _e54;
    } else {
        let _e57 = global.member.member_1;
        phi_108_ = _e57;
    }
    let _e59 = phi_108_;
    global_2 = vec4<f32>((fma(_e59.member_2.x, _e48.z, fma(_e59.member.x, _e48.x, (_e59.member_1.x * _e48.y))) + _e59.member_3.x), (fma(_e59.member_2.y, _e48.z, fma(_e59.member.y, _e48.x, (_e59.member_1.y * _e48.y))) + _e59.member_3.y), (fma(_e59.member_2.z, _e48.z, fma(_e59.member.z, _e48.x, (_e59.member_1.z * _e48.y))) + _e59.member_3.z), (fma(_e59.member_2.w, _e48.z, fma(_e59.member.w, _e48.x, (_e59.member_1.w * _e48.y))) + _e59.member_3.w));
    let _e102 = local[(_e49 % 16u)];
    global_5 = _e102;
    let _e105 = global.member.member_12;
    if ((_e105 & 1u) == 1u) {
        let _e109 = global_2[1u];
        global_2[1u] = -(_e109);
    }
    return;
}

@vertex 
fn line_vertex(@location(0) param: vec3<f32>, @location(1) param_1: u32, @builtin(view_index) param_2: i32) -> VertexOutput {
    global_3 = param;
    global_4 = param_1;
    global_1 = param_2;
    function();
    let _e9 = global_2.y;
    global_2.y = -(_e9);
    let _e11 = global_2;
    let _e12 = global_5;
    return VertexOutput(_e11, _e12);
}
