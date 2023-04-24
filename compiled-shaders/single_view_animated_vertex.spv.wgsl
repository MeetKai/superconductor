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

struct type_13 {
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

struct type_15 {
    member: type_13,
}

struct type_23 {
    member: vec4<f32>,
    member_1: vec4<f32>,
}

struct type_25 {
    member: array<type_23,2048u>,
}

struct VertexOutput {
    @builtin(position) member: vec4<f32>,
    @location(0) member_1: vec3<f32>,
    @location(1) member_2: vec3<f32>,
    @location(2) member_3: vec2<f32>,
    @location(3) member_4: vec2<f32>,
    @location(4) @interpolate(flat) member_5: u32,
    @location(5) @interpolate(flat) member_6: u32,
}

var<private> global: vec4<f32>;
var<private> global_1: vec4<f32>;
var<private> global_2: u32;
var<private> global_3: u32;
var<private> global_4: vec3<f32>;
var<private> global_5: vec3<f32>;
var<private> global_6: vec2<f32>;
@group(0) @binding(0) 
var<uniform> global_7: type_11;
@group(1) @binding(4) 
var<uniform> global_8: type_15;
var<private> global_9: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 1.0);
var<private> global_10: vec3<f32>;
var<private> global_11: vec3<f32>;
var<private> global_12: vec2<f32>;
var<private> global_13: u32;
var<private> global_14: vec2<f32>;
var<private> global_15: u32;
var<private> global_16: u32;
var<private> global_17: vec4<u32>;
var<private> global_18: vec4<f32>;
@group(2) @binding(0) 
var<uniform> global_19: type_25;

fn function() {
    let _e33 = global;
    let _e34 = global_1;
    let _e35 = global_16;
    let _e36 = global_2;
    let _e37 = global_3;
    let _e38 = global_4;
    let _e39 = global_5;
    let _e40 = global_6;
    let _e41 = global_17;
    let _e42 = global_18;
    let _e49 = (_e41 + vec4<u32>(_e35));
    let _e62 = (_e42 / vec4<f32>((((_e42.x + _e42.y) + _e42.z) + _e42.w)));
    let _e69 = global_19.member[_e49.x];
    let _e85 = fma(_e69.member_1.w, _e69.member_1.w, -(fma(_e69.member_1.z, _e69.member_1.z, fma(_e69.member_1.x, _e69.member_1.x, (_e69.member_1.y * _e69.member_1.y)))));
    let _e92 = (fma(_e38.z, _e69.member_1.z, fma(_e38.x, _e69.member_1.x, (_e38.y * _e69.member_1.y))) * 2.0);
    let _e108 = (_e69.member_1.w * 2.0);
    let _e116 = (vec3<f32>(_e69.member.x, _e69.member.y, _e69.member.z) + vec3<f32>((_e69.member.w * fma(fma(_e69.member_1.y, _e38.z, -((_e38.y * _e69.member_1.z))), _e108, fma(_e38.x, _e85, (_e69.member_1.x * _e92)))), (_e69.member.w * fma(fma(_e69.member_1.z, _e38.x, -((_e38.z * _e69.member_1.x))), _e108, fma(_e38.y, _e85, (_e69.member_1.y * _e92)))), (_e69.member.w * fma(fma(_e69.member_1.x, _e38.y, -((_e38.x * _e69.member_1.y))), _e108, fma(_e38.z, _e85, (_e69.member_1.z * _e92))))));
    let _e122 = global_19.member[_e49.y];
    let _e138 = fma(_e122.member_1.w, _e122.member_1.w, -(fma(_e122.member_1.z, _e122.member_1.z, fma(_e122.member_1.x, _e122.member_1.x, (_e122.member_1.y * _e122.member_1.y)))));
    let _e142 = (fma(_e38.z, _e122.member_1.z, fma(_e38.x, _e122.member_1.x, (_e38.y * _e122.member_1.y))) * 2.0);
    let _e158 = (_e122.member_1.w * 2.0);
    let _e167 = ((vec3<f32>(_e122.member.x, _e122.member.y, _e122.member.z) + vec3<f32>((_e122.member.w * fma(fma(_e122.member_1.y, _e38.z, -((_e38.y * _e122.member_1.z))), _e158, fma(_e38.x, _e138, (_e122.member_1.x * _e142)))), (_e122.member.w * fma(fma(_e122.member_1.z, _e38.x, -((_e38.z * _e122.member_1.x))), _e158, fma(_e38.y, _e138, (_e122.member_1.y * _e142)))), (_e122.member.w * fma(fma(_e122.member_1.x, _e38.y, -((_e38.x * _e122.member_1.y))), _e158, fma(_e38.z, _e138, (_e122.member_1.z * _e142)))))) * _e62.y);
    let _e176 = global_19.member[_e49.z];
    let _e192 = fma(_e176.member_1.w, _e176.member_1.w, -(fma(_e176.member_1.z, _e176.member_1.z, fma(_e176.member_1.x, _e176.member_1.x, (_e176.member_1.y * _e176.member_1.y)))));
    let _e196 = (fma(_e38.z, _e176.member_1.z, fma(_e38.x, _e176.member_1.x, (_e38.y * _e176.member_1.y))) * 2.0);
    let _e212 = (_e176.member_1.w * 2.0);
    let _e220 = (vec3<f32>(_e176.member.x, _e176.member.y, _e176.member.z) + vec3<f32>((_e176.member.w * fma(fma(_e176.member_1.y, _e38.z, -((_e38.y * _e176.member_1.z))), _e212, fma(_e38.x, _e192, (_e176.member_1.x * _e196)))), (_e176.member.w * fma(fma(_e176.member_1.z, _e38.x, -((_e38.z * _e176.member_1.x))), _e212, fma(_e38.y, _e192, (_e176.member_1.y * _e196)))), (_e176.member.w * fma(fma(_e176.member_1.x, _e38.y, -((_e38.x * _e176.member_1.y))), _e212, fma(_e38.z, _e192, (_e176.member_1.z * _e196))))));
    let _e229 = global_19.member[_e49.w];
    let _e245 = fma(_e229.member_1.w, _e229.member_1.w, -(fma(_e229.member_1.z, _e229.member_1.z, fma(_e229.member_1.x, _e229.member_1.x, (_e229.member_1.y * _e229.member_1.y)))));
    let _e249 = (fma(_e38.z, _e229.member_1.z, fma(_e38.x, _e229.member_1.x, (_e38.y * _e229.member_1.y))) * 2.0);
    let _e265 = (_e229.member_1.w * 2.0);
    let _e273 = (vec3<f32>(_e229.member.x, _e229.member.y, _e229.member.z) + vec3<f32>((_e229.member.w * fma(fma(_e229.member_1.y, _e38.z, -((_e38.y * _e229.member_1.z))), _e265, fma(_e38.x, _e245, (_e229.member_1.x * _e249)))), (_e229.member.w * fma(fma(_e229.member_1.z, _e38.x, -((_e38.z * _e229.member_1.x))), _e265, fma(_e38.y, _e245, (_e229.member_1.y * _e249)))), (_e229.member.w * fma(fma(_e229.member_1.x, _e38.y, -((_e38.x * _e229.member_1.y))), _e265, fma(_e38.z, _e245, (_e229.member_1.z * _e249))))));
    let _e283 = global_19.member[_e49.x].member_1;
    let _e292 = fma(_e283.w, _e283.w, -(fma(_e283.z, _e283.z, fma(_e283.x, _e283.x, (_e283.y * _e283.y)))));
    let _e299 = (fma(_e39.z, _e283.z, fma(_e39.x, _e283.x, (_e39.y * _e283.y))) * 2.0);
    let _e315 = (_e283.w * 2.0);
    let _e322 = global_19.member[_e49.y].member_1;
    let _e331 = fma(_e322.w, _e322.w, -(fma(_e322.z, _e322.z, fma(_e322.x, _e322.x, (_e322.y * _e322.y)))));
    let _e335 = (fma(_e39.z, _e322.z, fma(_e39.x, _e322.x, (_e39.y * _e322.y))) * 2.0);
    let _e351 = (_e322.w * 2.0);
    let _e364 = global_19.member[_e49.z].member_1;
    let _e373 = fma(_e364.w, _e364.w, -(fma(_e364.z, _e364.z, fma(_e364.x, _e364.x, (_e364.y * _e364.y)))));
    let _e377 = (fma(_e39.z, _e364.z, fma(_e39.x, _e364.x, (_e39.y * _e364.y))) * 2.0);
    let _e393 = (_e364.w * 2.0);
    let _e403 = global_19.member[_e49.w].member_1;
    let _e412 = fma(_e403.w, _e403.w, -(fma(_e403.z, _e403.z, fma(_e403.x, _e403.x, (_e403.y * _e403.y)))));
    let _e416 = (fma(_e39.z, _e403.z, fma(_e39.x, _e403.x, (_e39.y * _e403.y))) * 2.0);
    let _e432 = (_e403.w * 2.0);
    let _e436 = fma(fma(fma(_e403.y, _e39.z, -((_e39.y * _e403.z))), _e432, fma(_e39.x, _e412, (_e403.x * _e416))), _e62.w, fma(fma(fma(_e364.y, _e39.z, -((_e39.y * _e364.z))), _e393, fma(_e39.x, _e373, (_e364.x * _e377))), _e62.z, fma(fma(fma(_e283.y, _e39.z, -((_e39.y * _e283.z))), _e315, fma(_e39.x, _e292, (_e283.x * _e299))), _e62.x, (fma(fma(_e322.y, _e39.z, -((_e39.y * _e322.z))), _e351, fma(_e39.x, _e331, (_e322.x * _e335))) * _e62.y))));
    let _e437 = fma(fma(fma(_e403.z, _e39.x, -((_e39.z * _e403.x))), _e432, fma(_e39.y, _e412, (_e403.y * _e416))), _e62.w, fma(fma(fma(_e364.z, _e39.x, -((_e39.z * _e364.x))), _e393, fma(_e39.y, _e373, (_e364.y * _e377))), _e62.z, fma(fma(fma(_e283.z, _e39.x, -((_e39.z * _e283.x))), _e315, fma(_e39.y, _e292, (_e283.y * _e299))), _e62.x, (fma(fma(_e322.z, _e39.x, -((_e39.z * _e322.x))), _e351, fma(_e39.y, _e331, (_e322.y * _e335))) * _e62.y))));
    let _e438 = fma(fma(fma(_e403.x, _e39.y, -((_e39.x * _e403.y))), _e432, fma(_e39.z, _e412, (_e403.z * _e416))), _e62.w, fma(fma(fma(_e364.x, _e39.y, -((_e39.x * _e364.y))), _e393, fma(_e39.z, _e373, (_e364.z * _e377))), _e62.z, fma(fma(fma(_e283.x, _e39.y, -((_e39.x * _e283.y))), _e315, fma(_e39.z, _e292, (_e283.z * _e299))), _e62.x, (fma(fma(_e322.x, _e39.y, -((_e39.x * _e322.y))), _e351, fma(_e39.z, _e331, (_e322.z * _e335))) * _e62.y))));
    let _e439 = (_e33.w * fma(_e273.x, _e62.w, fma(_e220.x, _e62.z, fma(_e116.x, _e62.x, _e167.x))));
    let _e440 = (_e33.w * fma(_e273.y, _e62.w, fma(_e220.y, _e62.z, fma(_e116.y, _e62.x, _e167.y))));
    let _e441 = (_e33.w * fma(_e273.z, _e62.w, fma(_e220.z, _e62.z, fma(_e116.z, _e62.x, _e167.z))));
    let _e450 = fma(_e34.w, _e34.w, -(fma(_e34.z, _e34.z, fma(_e34.x, _e34.x, (_e34.y * _e34.y)))));
    let _e454 = (fma(_e441, _e34.z, fma(_e439, _e34.x, (_e440 * _e34.y))) * 2.0);
    let _e470 = (_e34.w * 2.0);
    let _e474 = (_e33.x + fma(fma(_e34.y, _e441, -((_e440 * _e34.z))), _e470, fma(_e439, _e450, (_e34.x * _e454))));
    let _e475 = (_e33.y + fma(fma(_e34.z, _e439, -((_e441 * _e34.x))), _e470, fma(_e440, _e450, (_e34.y * _e454))));
    let _e476 = (_e33.z + fma(fma(_e34.x, _e440, -((_e439 * _e34.y))), _e470, fma(_e441, _e450, (_e34.z * _e454))));
    let _e480 = global_7.member.member;
    global_9 = vec4<f32>((fma(_e480.member_2.x, _e476, fma(_e480.member.x, _e474, (_e480.member_1.x * _e475))) + _e480.member_3.x), (fma(_e480.member_2.y, _e476, fma(_e480.member.y, _e474, (_e480.member_1.y * _e475))) + _e480.member_3.y), (fma(_e480.member_2.z, _e476, fma(_e480.member.z, _e474, (_e480.member_1.z * _e475))) + _e480.member_3.z), (fma(_e480.member_2.w, _e476, fma(_e480.member.w, _e474, (_e480.member_1.w * _e475))) + _e480.member_3.w));
    global_10 = vec3<f32>(_e474, _e475, _e476);
    let _e521 = (fma(_e438, _e34.z, fma(_e436, _e34.x, (_e437 * _e34.y))) * 2.0);
    global_11 = vec3<f32>(fma(fma(_e34.y, _e438, -((_e437 * _e34.z))), _e470, fma(_e436, _e450, (_e34.x * _e521))), fma(fma(_e34.z, _e436, -((_e438 * _e34.x))), _e470, fma(_e437, _e450, (_e34.y * _e521))), fma(fma(_e34.x, _e437, -((_e436 * _e34.y))), _e470, fma(_e438, _e450, (_e34.z * _e521))));
    let _e542 = global_8.member.member_1;
    let _e544 = global_8.member.member_6;
    let _e546 = global_8.member.member_2;
    let _e547 = sin(_e544);
    let _e548 = cos(_e544);
    global_12 = (_e542 + vec2<f32>((fma(_e548, _e546.x, (-(_e547) * _e546.y)) * _e40.x), (fma(_e547, _e546.x, (_e548 * _e546.y)) * _e40.y)));
    global_13 = _e36;
    global_14 = vec2<f32>(0.0, 0.0);
    global_15 = _e37;
    let _e564 = global_7.member.member_12;
    if ((_e564 & 1u) == 1u) {
        let _e568 = global_9[1u];
        global_9[1u] = -(_e568);
    }
    return;
}

@vertex 
fn single_view__animated_vertex(@location(0) param: vec4<f32>, @location(1) param_1: vec4<f32>, @location(2) param_2: u32, @location(3) param_3: u32, @location(4) param_4: u32, @location(5) param_5: vec3<f32>, @location(6) param_6: vec3<f32>, @location(7) param_7: vec2<f32>, @location(8) param_8: vec4<u32>, @location(9) param_9: vec4<f32>) -> VertexOutput {
    global = param;
    global_1 = param_1;
    global_16 = param_2;
    global_2 = param_3;
    global_3 = param_4;
    global_4 = param_5;
    global_5 = param_6;
    global_6 = param_7;
    global_17 = param_8;
    global_18 = param_9;
    function();
    let _e28 = global_9.y;
    global_9.y = -(_e28);
    let _e30 = global_9;
    let _e31 = global_10;
    let _e32 = global_11;
    let _e33 = global_12;
    let _e34 = global_14;
    let _e35 = global_13;
    let _e36 = global_15;
    return VertexOutput(_e30, _e31, _e32, _e33, _e34, _e35, _e36);
}
