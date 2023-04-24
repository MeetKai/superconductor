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

struct type_15 {
    member: i32,
    member_1: vec2<f32>,
}

@group(0) @binding(0) 
var<uniform> global: type_10;
var<private> global_1: vec4<f32>;
var<private> global_2: vec2<f32>;
@group(1) @binding(0) 
var global_3: sampler;
@group(1) @binding(1) 
var global_4: texture_2d_array<f32>;

fn function() {
    var phi_162_: type_15;
    var phi_183_: vec3<f32>;
    var phi_192_: vec3<f32>;

    let _e29 = global_2;
    if (_e29.x > 0.5) {
        phi_162_ = type_15(1, vec2<f32>(fma(_e29.x, 2.0, -1.0), _e29.y));
    } else {
        phi_162_ = type_15(0, vec2<f32>((_e29.x * 2.0), _e29.y));
    }
    let _e41 = phi_162_;
    let _e47 = vec3<f32>(_e41.member_1.x, _e41.member_1.y, f32(_e41.member));
    let _e53 = textureSample(global_4, global_3, vec2<f32>(_e47.x, _e47.y), i32(_e47.z));
    let _e60 = global.member.member_12;
    if (((_e60 & 2u) == 2u) != true) {
        phi_183_ = min(vec3<f32>(max(((_e53.x * fma(2.509999990463257, _e53.x, 0.029999999329447746)) / fma(_e53.x, fma(2.430000066757202, _e53.x, 0.5899999737739563), 0.14000000059604645)), 0.0), max(((_e53.y * fma(2.509999990463257, _e53.y, 0.029999999329447746)) / fma(_e53.y, fma(2.430000066757202, _e53.y, 0.5899999737739563), 0.14000000059604645)), 0.0), max(((_e53.z * fma(2.509999990463257, _e53.z, 0.029999999329447746)) / fma(_e53.z, fma(2.430000066757202, _e53.z, 0.5899999737739563), 0.14000000059604645)), 0.0)), vec3<f32>(1.0, 1.0, 1.0));
    } else {
        phi_183_ = vec3<f32>(_e53.x, _e53.y, _e53.z);
    }
    let _e85 = phi_183_;
    if (((_e60 & 4u) == 4u) != true) {
        phi_192_ = pow(_e85, vec3<f32>(0.45454543828964233, 0.45454543828964233, 0.45454543828964233));
    } else {
        phi_192_ = _e85;
    }
    let _e91 = phi_192_;
    global_1 = vec4<f32>(_e91.x, _e91.y, _e91.z, 1.0);
    return;
}

@fragment 
fn tonemap(@location(0) param: vec2<f32>) -> @location(0) vec4<f32> {
    global_2 = param;
    function();
    let _e3 = global_1;
    return _e3;
}
