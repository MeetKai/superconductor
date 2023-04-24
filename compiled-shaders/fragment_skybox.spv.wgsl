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
var<private> global_1: vec4<f32>;
var<private> global_2: vec3<f32>;
@group(0) @binding(1) 
var global_3: sampler;
@group(0) @binding(2) 
var global_4: texture_cube<f32>;

fn function() {
    var phi_150_: vec3<f32>;
    var phi_158_: vec3<f32>;

    let _e23 = global_2;
    let _e24 = textureSampleLevel(global_4, global_3, _e23, 0.0);
    let _e31 = global.member.member_12;
    if ((_e31 & 2u) == 2u) {
        phi_150_ = min(vec3<f32>(max(((_e24.x * fma(2.509999990463257, _e24.x, 0.029999999329447746)) / fma(_e24.x, fma(2.430000066757202, _e24.x, 0.5899999737739563), 0.14000000059604645)), 0.0), max(((_e24.y * fma(2.509999990463257, _e24.y, 0.029999999329447746)) / fma(_e24.y, fma(2.430000066757202, _e24.y, 0.5899999737739563), 0.14000000059604645)), 0.0), max(((_e24.z * fma(2.509999990463257, _e24.z, 0.029999999329447746)) / fma(_e24.z, fma(2.430000066757202, _e24.z, 0.5899999737739563), 0.14000000059604645)), 0.0)), vec3<f32>(1.0, 1.0, 1.0));
    } else {
        phi_150_ = vec3<f32>(_e24.x, _e24.y, _e24.z);
    }
    let _e55 = phi_150_;
    if ((_e31 & 4u) == 4u) {
        phi_158_ = pow(_e55, vec3<f32>(0.45454543828964233, 0.45454543828964233, 0.45454543828964233));
    } else {
        phi_158_ = _e55;
    }
    let _e60 = phi_158_;
    global_1 = vec4<f32>(_e60.x, _e60.y, _e60.z, 1.0);
    return;
}

@fragment 
fn fragment_skybox(@location(0) param: vec3<f32>) -> @location(0) vec4<f32> {
    global_2 = param;
    function();
    let _e3 = global_1;
    return _e3;
}
