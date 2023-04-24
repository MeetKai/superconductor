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

struct type_25 {
    member: vec4<f32>,
    member_1: vec4<f32>,
}

struct type_27 {
    member: array<type_25,2048u>,
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
@group(0) @binding(0) 
var<uniform> global_7: type_13;
@group(1) @binding(4) 
var<uniform> global_8: type_17;
var<private> global_9: i32;
var<private> global_10: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 1.0);
var<private> global_11: vec3<f32>;
var<private> global_12: vec3<f32>;
var<private> global_13: vec2<f32>;
var<private> global_14: u32;
var<private> global_15: vec2<f32>;
var<private> global_16: u32;
var<private> global_17: u32;
var<private> global_18: vec4<u32>;
var<private> global_19: vec4<f32>;
@group(2) @binding(0) 
var<uniform> global_20: type_27;

fn function() {
    var phi_568_: type_11;

    let _e35 = global;
    let _e36 = global_1;
    let _e37 = global_17;
    let _e38 = global_2;
    let _e39 = global_3;
    let _e40 = global_4;
    let _e41 = global_5;
    let _e42 = global_6;
    let _e43 = global_18;
    let _e44 = global_19;
    let _e46 = global_9;
    let _e52 = (_e43 + vec4<u32>(_e37));
    let _e65 = (_e44 / vec4<f32>((((_e44.x + _e44.y) + _e44.z) + _e44.w)));
    let _e72 = global_20.member[_e52.x];
    let _e88 = fma(_e72.member_1.w, _e72.member_1.w, -(fma(_e72.member_1.z, _e72.member_1.z, fma(_e72.member_1.x, _e72.member_1.x, (_e72.member_1.y * _e72.member_1.y)))));
    let _e95 = (fma(_e40.z, _e72.member_1.z, fma(_e40.x, _e72.member_1.x, (_e40.y * _e72.member_1.y))) * 2.0);
    let _e111 = (_e72.member_1.w * 2.0);
    let _e119 = (vec3<f32>(_e72.member.x, _e72.member.y, _e72.member.z) + vec3<f32>((_e72.member.w * fma(fma(_e72.member_1.y, _e40.z, -((_e40.y * _e72.member_1.z))), _e111, fma(_e40.x, _e88, (_e72.member_1.x * _e95)))), (_e72.member.w * fma(fma(_e72.member_1.z, _e40.x, -((_e40.z * _e72.member_1.x))), _e111, fma(_e40.y, _e88, (_e72.member_1.y * _e95)))), (_e72.member.w * fma(fma(_e72.member_1.x, _e40.y, -((_e40.x * _e72.member_1.y))), _e111, fma(_e40.z, _e88, (_e72.member_1.z * _e95))))));
    let _e125 = global_20.member[_e52.y];
    let _e141 = fma(_e125.member_1.w, _e125.member_1.w, -(fma(_e125.member_1.z, _e125.member_1.z, fma(_e125.member_1.x, _e125.member_1.x, (_e125.member_1.y * _e125.member_1.y)))));
    let _e145 = (fma(_e40.z, _e125.member_1.z, fma(_e40.x, _e125.member_1.x, (_e40.y * _e125.member_1.y))) * 2.0);
    let _e161 = (_e125.member_1.w * 2.0);
    let _e170 = ((vec3<f32>(_e125.member.x, _e125.member.y, _e125.member.z) + vec3<f32>((_e125.member.w * fma(fma(_e125.member_1.y, _e40.z, -((_e40.y * _e125.member_1.z))), _e161, fma(_e40.x, _e141, (_e125.member_1.x * _e145)))), (_e125.member.w * fma(fma(_e125.member_1.z, _e40.x, -((_e40.z * _e125.member_1.x))), _e161, fma(_e40.y, _e141, (_e125.member_1.y * _e145)))), (_e125.member.w * fma(fma(_e125.member_1.x, _e40.y, -((_e40.x * _e125.member_1.y))), _e161, fma(_e40.z, _e141, (_e125.member_1.z * _e145)))))) * _e65.y);
    let _e179 = global_20.member[_e52.z];
    let _e195 = fma(_e179.member_1.w, _e179.member_1.w, -(fma(_e179.member_1.z, _e179.member_1.z, fma(_e179.member_1.x, _e179.member_1.x, (_e179.member_1.y * _e179.member_1.y)))));
    let _e199 = (fma(_e40.z, _e179.member_1.z, fma(_e40.x, _e179.member_1.x, (_e40.y * _e179.member_1.y))) * 2.0);
    let _e215 = (_e179.member_1.w * 2.0);
    let _e223 = (vec3<f32>(_e179.member.x, _e179.member.y, _e179.member.z) + vec3<f32>((_e179.member.w * fma(fma(_e179.member_1.y, _e40.z, -((_e40.y * _e179.member_1.z))), _e215, fma(_e40.x, _e195, (_e179.member_1.x * _e199)))), (_e179.member.w * fma(fma(_e179.member_1.z, _e40.x, -((_e40.z * _e179.member_1.x))), _e215, fma(_e40.y, _e195, (_e179.member_1.y * _e199)))), (_e179.member.w * fma(fma(_e179.member_1.x, _e40.y, -((_e40.x * _e179.member_1.y))), _e215, fma(_e40.z, _e195, (_e179.member_1.z * _e199))))));
    let _e232 = global_20.member[_e52.w];
    let _e248 = fma(_e232.member_1.w, _e232.member_1.w, -(fma(_e232.member_1.z, _e232.member_1.z, fma(_e232.member_1.x, _e232.member_1.x, (_e232.member_1.y * _e232.member_1.y)))));
    let _e252 = (fma(_e40.z, _e232.member_1.z, fma(_e40.x, _e232.member_1.x, (_e40.y * _e232.member_1.y))) * 2.0);
    let _e268 = (_e232.member_1.w * 2.0);
    let _e276 = (vec3<f32>(_e232.member.x, _e232.member.y, _e232.member.z) + vec3<f32>((_e232.member.w * fma(fma(_e232.member_1.y, _e40.z, -((_e40.y * _e232.member_1.z))), _e268, fma(_e40.x, _e248, (_e232.member_1.x * _e252)))), (_e232.member.w * fma(fma(_e232.member_1.z, _e40.x, -((_e40.z * _e232.member_1.x))), _e268, fma(_e40.y, _e248, (_e232.member_1.y * _e252)))), (_e232.member.w * fma(fma(_e232.member_1.x, _e40.y, -((_e40.x * _e232.member_1.y))), _e268, fma(_e40.z, _e248, (_e232.member_1.z * _e252))))));
    let _e286 = global_20.member[_e52.x].member_1;
    let _e295 = fma(_e286.w, _e286.w, -(fma(_e286.z, _e286.z, fma(_e286.x, _e286.x, (_e286.y * _e286.y)))));
    let _e302 = (fma(_e41.z, _e286.z, fma(_e41.x, _e286.x, (_e41.y * _e286.y))) * 2.0);
    let _e318 = (_e286.w * 2.0);
    let _e325 = global_20.member[_e52.y].member_1;
    let _e334 = fma(_e325.w, _e325.w, -(fma(_e325.z, _e325.z, fma(_e325.x, _e325.x, (_e325.y * _e325.y)))));
    let _e338 = (fma(_e41.z, _e325.z, fma(_e41.x, _e325.x, (_e41.y * _e325.y))) * 2.0);
    let _e354 = (_e325.w * 2.0);
    let _e367 = global_20.member[_e52.z].member_1;
    let _e376 = fma(_e367.w, _e367.w, -(fma(_e367.z, _e367.z, fma(_e367.x, _e367.x, (_e367.y * _e367.y)))));
    let _e380 = (fma(_e41.z, _e367.z, fma(_e41.x, _e367.x, (_e41.y * _e367.y))) * 2.0);
    let _e396 = (_e367.w * 2.0);
    let _e406 = global_20.member[_e52.w].member_1;
    let _e415 = fma(_e406.w, _e406.w, -(fma(_e406.z, _e406.z, fma(_e406.x, _e406.x, (_e406.y * _e406.y)))));
    let _e419 = (fma(_e41.z, _e406.z, fma(_e41.x, _e406.x, (_e41.y * _e406.y))) * 2.0);
    let _e435 = (_e406.w * 2.0);
    let _e439 = fma(fma(fma(_e406.y, _e41.z, -((_e41.y * _e406.z))), _e435, fma(_e41.x, _e415, (_e406.x * _e419))), _e65.w, fma(fma(fma(_e367.y, _e41.z, -((_e41.y * _e367.z))), _e396, fma(_e41.x, _e376, (_e367.x * _e380))), _e65.z, fma(fma(fma(_e286.y, _e41.z, -((_e41.y * _e286.z))), _e318, fma(_e41.x, _e295, (_e286.x * _e302))), _e65.x, (fma(fma(_e325.y, _e41.z, -((_e41.y * _e325.z))), _e354, fma(_e41.x, _e334, (_e325.x * _e338))) * _e65.y))));
    let _e440 = fma(fma(fma(_e406.z, _e41.x, -((_e41.z * _e406.x))), _e435, fma(_e41.y, _e415, (_e406.y * _e419))), _e65.w, fma(fma(fma(_e367.z, _e41.x, -((_e41.z * _e367.x))), _e396, fma(_e41.y, _e376, (_e367.y * _e380))), _e65.z, fma(fma(fma(_e286.z, _e41.x, -((_e41.z * _e286.x))), _e318, fma(_e41.y, _e295, (_e286.y * _e302))), _e65.x, (fma(fma(_e325.z, _e41.x, -((_e41.z * _e325.x))), _e354, fma(_e41.y, _e334, (_e325.y * _e338))) * _e65.y))));
    let _e441 = fma(fma(fma(_e406.x, _e41.y, -((_e41.x * _e406.y))), _e435, fma(_e41.z, _e415, (_e406.z * _e419))), _e65.w, fma(fma(fma(_e367.x, _e41.y, -((_e41.x * _e367.y))), _e396, fma(_e41.z, _e376, (_e367.z * _e380))), _e65.z, fma(fma(fma(_e286.x, _e41.y, -((_e41.x * _e286.y))), _e318, fma(_e41.z, _e295, (_e286.z * _e302))), _e65.x, (fma(fma(_e325.x, _e41.y, -((_e41.x * _e325.y))), _e354, fma(_e41.z, _e334, (_e325.z * _e338))) * _e65.y))));
    let _e442 = (_e35.w * fma(_e276.x, _e65.w, fma(_e223.x, _e65.z, fma(_e119.x, _e65.x, _e170.x))));
    let _e443 = (_e35.w * fma(_e276.y, _e65.w, fma(_e223.y, _e65.z, fma(_e119.y, _e65.x, _e170.y))));
    let _e444 = (_e35.w * fma(_e276.z, _e65.w, fma(_e223.z, _e65.z, fma(_e119.z, _e65.x, _e170.z))));
    let _e453 = fma(_e36.w, _e36.w, -(fma(_e36.z, _e36.z, fma(_e36.x, _e36.x, (_e36.y * _e36.y)))));
    let _e457 = (fma(_e444, _e36.z, fma(_e442, _e36.x, (_e443 * _e36.y))) * 2.0);
    let _e473 = (_e36.w * 2.0);
    let _e477 = (_e35.x + fma(fma(_e36.y, _e444, -((_e443 * _e36.z))), _e473, fma(_e442, _e453, (_e36.x * _e457))));
    let _e478 = (_e35.y + fma(fma(_e36.z, _e442, -((_e444 * _e36.x))), _e473, fma(_e443, _e453, (_e36.y * _e457))));
    let _e479 = (_e35.z + fma(fma(_e36.x, _e443, -((_e442 * _e36.y))), _e473, fma(_e444, _e453, (_e36.z * _e457))));
    if (_e46 == 0) {
        let _e487 = global_7.member.member;
        phi_568_ = _e487;
    } else {
        let _e484 = global_7.member.member_1;
        phi_568_ = _e484;
    }
    let _e489 = phi_568_;
    global_10 = vec4<f32>((fma(_e489.member_2.x, _e479, fma(_e489.member.x, _e477, (_e489.member_1.x * _e478))) + _e489.member_3.x), (fma(_e489.member_2.y, _e479, fma(_e489.member.y, _e477, (_e489.member_1.y * _e478))) + _e489.member_3.y), (fma(_e489.member_2.z, _e479, fma(_e489.member.z, _e477, (_e489.member_1.z * _e478))) + _e489.member_3.z), (fma(_e489.member_2.w, _e479, fma(_e489.member.w, _e477, (_e489.member_1.w * _e478))) + _e489.member_3.w));
    global_11 = vec3<f32>(_e477, _e478, _e479);
    let _e530 = (fma(_e441, _e36.z, fma(_e439, _e36.x, (_e440 * _e36.y))) * 2.0);
    global_12 = vec3<f32>(fma(fma(_e36.y, _e441, -((_e440 * _e36.z))), _e473, fma(_e439, _e453, (_e36.x * _e530))), fma(fma(_e36.z, _e439, -((_e441 * _e36.x))), _e473, fma(_e440, _e453, (_e36.y * _e530))), fma(fma(_e36.x, _e440, -((_e439 * _e36.y))), _e473, fma(_e441, _e453, (_e36.z * _e530))));
    let _e551 = global_8.member.member_1;
    let _e553 = global_8.member.member_6;
    let _e555 = global_8.member.member_2;
    let _e556 = sin(_e553);
    let _e557 = cos(_e553);
    global_13 = (_e551 + vec2<f32>((fma(_e557, _e555.x, (-(_e556) * _e555.y)) * _e42.x), (fma(_e556, _e555.x, (_e557 * _e555.y)) * _e42.y)));
    global_14 = _e38;
    global_15 = vec2<f32>(0.0, 0.0);
    global_16 = _e39;
    let _e573 = global_7.member.member_12;
    if ((_e573 & 1u) == 1u) {
        let _e577 = global_10[1u];
        global_10[1u] = -(_e577);
    }
    return;
}

@vertex 
fn animated_vertex(@location(0) param: vec4<f32>, @location(1) param_1: vec4<f32>, @location(2) param_2: u32, @location(3) param_3: u32, @location(4) param_4: u32, @location(5) param_5: vec3<f32>, @location(6) param_6: vec3<f32>, @location(7) param_7: vec2<f32>, @location(8) param_8: vec4<u32>, @location(9) param_9: vec4<f32>, @builtin(view_index) param_10: i32) -> VertexOutput {
    global = param;
    global_1 = param_1;
    global_17 = param_2;
    global_2 = param_3;
    global_3 = param_4;
    global_4 = param_5;
    global_5 = param_6;
    global_6 = param_7;
    global_18 = param_8;
    global_19 = param_9;
    global_9 = param_10;
    function();
    let _e30 = global_10.y;
    global_10.y = -(_e30);
    let _e32 = global_10;
    let _e33 = global_11;
    let _e34 = global_12;
    let _e35 = global_13;
    let _e36 = global_15;
    let _e37 = global_14;
    let _e38 = global_16;
    return VertexOutput(_e32, _e33, _e34, _e35, _e36, _e37, _e38);
}
