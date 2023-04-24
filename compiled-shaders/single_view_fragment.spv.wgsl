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

@group(0) @binding(0) 
var<uniform> global: type_11;
@group(1) @binding(4) 
var<uniform> global_1: type_15;
var<private> global_2: vec3<f32>;
var<private> global_3: vec3<f32>;
var<private> global_4: vec2<f32>;
var<private> global_5: vec2<f32>;
var<private> global_6: u32;
var<private> global_7: bool;
@group(0) @binding(1) 
var global_8: sampler;
@group(0) @binding(3) 
var global_9: texture_3d<f32>;
@group(0) @binding(4) 
var global_10: texture_3d<f32>;
@group(0) @binding(5) 
var global_11: texture_3d<f32>;
@group(0) @binding(6) 
var global_12: texture_3d<f32>;
@group(0) @binding(7) 
var global_13: texture_2d<f32>;
@group(0) @binding(8) 
var global_14: texture_2d<f32>;
@group(0) @binding(9) 
var global_15: texture_2d<f32>;
@group(0) @binding(10) 
var global_16: texture_2d<f32>;
@group(1) @binding(5) 
var global_17: sampler;
@group(1) @binding(0) 
var global_18: texture_2d<f32>;
@group(1) @binding(2) 
var global_19: texture_2d<f32>;
@group(1) @binding(3) 
var global_20: texture_2d<f32>;
var<private> global_21: vec4<f32>;
@group(1) @binding(1) 
var global_22: texture_2d<f32>;
var<private> global_23: u32;

fn function() {
    var phi_1054_: array<vec3<f32>,4u>;
    var phi_1412_: vec3<f32>;
    var phi_2046_: f32;
    var phi_1208_: vec3<f32>;
    var phi_1216_: vec3<f32>;
    var phi_1108_: vec3<f32>;

    let _e70 = global_2;
    let _e71 = global_3;
    let _e72 = global_4;
    let _e73 = global_5;
    let _e74 = global_6;
    let _e76 = global_7;
    if (_e74 == 0u) {
        let _e110 = global.member.member_13;
        let _e113 = global.member.member_14;
        let _e116 = global.member.member_15;
        let _e119 = global.member.member_16;
        let _e122 = global.member.member_17;
        let _e125 = global.member.member_18;
        let _e134 = (vec3<f32>((_e70.x - _e110), (_e70.y - _e113), (_e70.z - _e116)) / vec3<f32>(_e119, _e122, _e125));
        let _e135 = textureSampleLevel(global_9, global_8, _e134, 0.0);
        let _e140 = textureSampleLevel(global_10, global_8, _e134, 0.0);
        let _e148 = textureSampleLevel(global_11, global_8, _e134, 0.0);
        let _e156 = textureSampleLevel(global_12, global_8, _e134, 0.0);
        phi_1054_ = array<vec3<f32>,4u>(vec3<f32>(_e135.x, _e135.y, _e135.z), vec3<f32>(fma(_e140.x, 2.007874011993408, -1.0078740119934082), fma(_e140.y, 2.007874011993408, -1.0078740119934082), fma(_e140.z, 2.007874011993408, -1.0078740119934082)), vec3<f32>(fma(_e148.x, 2.007874011993408, -1.0078740119934082), fma(_e148.y, 2.007874011993408, -1.0078740119934082), fma(_e148.z, 2.007874011993408, -1.0078740119934082)), vec3<f32>(fma(_e156.x, 2.007874011993408, -1.0078740119934082), fma(_e156.y, 2.007874011993408, -1.0078740119934082), fma(_e156.z, 2.007874011993408, -1.0078740119934082)));
    } else {
        let _e78 = textureSampleLevel(global_13, global_8, _e73, 0.0);
        let _e83 = textureSampleLevel(global_14, global_8, _e73, 0.0);
        let _e91 = textureSampleLevel(global_15, global_8, _e73, 0.0);
        let _e99 = textureSampleLevel(global_16, global_8, _e73, 0.0);
        phi_1054_ = array<vec3<f32>,4u>(vec3<f32>(_e78.x, _e78.y, _e78.z), vec3<f32>(fma(_e83.x, 2.007874011993408, -1.0078740119934082), fma(_e83.y, 2.007874011993408, -1.0078740119934082), fma(_e83.z, 2.007874011993408, -1.0078740119934082)), vec3<f32>(fma(_e91.x, 2.007874011993408, -1.0078740119934082), fma(_e91.y, 2.007874011993408, -1.0078740119934082), fma(_e91.z, 2.007874011993408, -1.0078740119934082)), vec3<f32>(fma(_e99.x, 2.007874011993408, -1.0078740119934082), fma(_e99.y, 2.007874011993408, -1.0078740119934082), fma(_e99.z, 2.007874011993408, -1.0078740119934082)));
    }
    let _e166 = phi_1054_;
    let _e167 = textureSample(global_18, global_17, _e72);
    let _e170 = global_1.member.member;
    let _e173 = (_e167.x * _e170.x);
    let _e176 = (_e167.y * _e170.y);
    let _e179 = (_e167.z * _e170.z);
    let _e180 = textureSample(global_20, global_17, _e72);
    let _e185 = global_1.member.member_3;
    let _e187 = global_1.member.member_4;
    let _e189 = global_1.member.member_5;
    let _e192 = textureSample(global_19, global_17, _e72);
    let _e196 = global_1.member.member_7;
    let _e197 = (_e192.z * _e196);
    let _e201 = global_1.member.member_8;
    let _e202 = vec3<f32>(_e173, _e176, _e179);
    let _e205 = global_1.member.member_10;
    if ((_e205 & 1u) == 1u) {
        let _e591 = global.member.member_12;
        if ((_e591 & 4u) == 4u) {
            phi_1108_ = pow(_e202, vec3<f32>(0.45454543828964233, 0.45454543828964233, 0.45454543828964233));
        } else {
            phi_1108_ = _e202;
        }
        let _e596 = phi_1108_;
        global_21 = vec4<f32>(_e596.x, _e596.y, _e596.z, 1.0);
    } else {
        let _e210 = global.member.member_6;
        let _e213 = global.member.member_7;
        let _e216 = global.member.member_8;
        let _e218 = (_e210 - _e70.x);
        let _e220 = (_e213 - _e70.y);
        let _e222 = (_e216 - _e70.z);
        let _e227 = (1.0 / sqrt(fma(_e222, _e222, fma(_e218, _e218, (_e220 * _e220)))));
        let _e228 = (_e218 * _e227);
        let _e229 = (_e220 * _e227);
        let _e230 = (_e222 * _e227);
        let _e234 = global_1.member.member_9;
        let _e242 = (1.0 / sqrt(fma(_e71.z, _e71.z, fma(_e71.x, _e71.x, (_e71.y * _e71.y)))));
        if (_e76 != true) {
            phi_1412_ = vec3<f32>(-((_e71.x * _e242)), -((_e71.y * _e242)), -((_e71.z * _e242)));
        } else {
            phi_1412_ = (_e71 * _e242);
        }
        let _e253 = phi_1412_;
        let _e254 = textureSample(global_22, global_17, _e72);
        let _e260 = fma(_e254.z, 2.007874011993408, -1.0078740119934082);
        let _e261 = (fma(_e254.x, 2.007874011993408, -1.0078740119934082) * _e234);
        let _e262 = (fma(_e254.y, 2.007874011993408, -1.0078740119934082) * _e234);
        let _e267 = (1.0 / sqrt(fma(_e260, _e260, fma(_e261, _e261, (_e262 * _e262)))));
        let _e268 = (_e261 * _e267);
        let _e269 = (_e262 * _e267);
        let _e270 = (_e260 * _e267);
        let _e271 = -(vec3<f32>(_e228, _e229, _e230));
        let _e272 = dpdx(_e271);
        let _e273 = dpdy(_e271);
        let _e274 = dpdx(_e72);
        let _e275 = dpdy(_e72);
        let _e282 = fma(_e273.y, _e253.z, -((_e253.y * _e273.z)));
        let _e287 = fma(_e273.z, _e253.x, -((_e253.z * _e273.x)));
        let _e290 = fma(_e273.x, _e253.y, -((_e253.x * _e273.y)));
        let _e304 = (_e275 * fma(_e253.y, _e272.z, -((_e272.y * _e253.z))));
        let _e307 = (_e275 * fma(_e253.z, _e272.x, -((_e272.z * _e253.x))));
        let _e310 = (_e275 * fma(_e253.x, _e272.y, -((_e272.x * _e253.y))));
        let _e313 = fma(_e282, _e274.x, _e304.x);
        let _e314 = fma(_e287, _e274.x, _e307.x);
        let _e315 = fma(_e290, _e274.x, _e310.x);
        let _e317 = fma(_e282, _e274.y, _e304.y);
        let _e318 = fma(_e287, _e274.y, _e307.y);
        let _e319 = fma(_e290, _e274.y, _e310.y);
        let _e328 = (1.0 / sqrt(max(fma(_e315, _e315, fma(_e313, _e313, (_e314 * _e314))), fma(_e319, _e319, fma(_e317, _e317, (_e318 * _e318))))));
        let _e341 = fma(_e253.x, _e270, fma((_e313 * _e328), _e268, ((_e317 * _e328) * _e269)));
        let _e342 = fma(_e253.y, _e270, fma((_e314 * _e328), _e268, ((_e318 * _e328) * _e269)));
        let _e343 = fma(_e253.z, _e270, fma((_e315 * _e328), _e268, ((_e319 * _e328) * _e269)));
        let _e348 = (1.0 / sqrt(fma(_e343, _e343, fma(_e341, _e341, (_e342 * _e342)))));
        let _e349 = (_e341 * _e348);
        let _e350 = (_e342 * _e348);
        let _e351 = (_e343 * _e348);
        let _e356 = fma(-(_e192.z), _e196, 1.0);
        let _e383 = sqrt(fma(_e166[3].x, _e166[3].x, fma(_e166[1].x, _e166[1].x, (_e166[2].x * _e166[2].x))));
        let _e386 = ((1.0 - _e383) / (1.0 + _e383));
        let _e387 = fma(2.0, _e383, 1.0);
        let _e404 = sqrt(fma(_e166[3].y, _e166[3].y, fma(_e166[1].y, _e166[1].y, (_e166[2].y * _e166[2].y))));
        let _e407 = ((1.0 - _e404) / (1.0 + _e404));
        let _e408 = fma(2.0, _e404, 1.0);
        let _e425 = sqrt(fma(_e166[3].z, _e166[3].z, fma(_e166[1].z, _e166[1].z, (_e166[2].z * _e166[2].z))));
        let _e428 = ((1.0 - _e425) / (1.0 + _e425));
        let _e429 = fma(2.0, _e425, 1.0);
        let _e450 = (((_e166[1].x + _e166[1].y) + _e166[1].z) * 0.3333333432674408);
        let _e451 = (((_e166[2].x + _e166[2].y) + _e166[2].z) * 0.3333333432674408);
        let _e452 = (((_e166[3].x + _e166[3].y) + _e166[3].z) * 0.3333333432674408);
        let _e456 = sqrt(fma(_e452, _e452, fma(_e450, _e450, (_e451 * _e451))));
        let _e461 = fma(-(fma(-(_e192.y), _e201, 1.0)), sqrt(_e456), 1.0);
        let _e462 = (_e461 * _e461);
        let _e463 = (_e450 / _e456);
        let _e464 = (_e451 / _e456);
        let _e465 = (_e452 / _e456);
        let _e466 = fma(_e218, _e227, _e463);
        let _e467 = fma(_e220, _e227, _e464);
        let _e468 = fma(_e222, _e227, _e465);
        let _e473 = (1.0 / sqrt(fma(_e468, _e468, fma(_e466, _e466, (_e467 * _e467)))));
        let _e474 = (_e466 * _e473);
        let _e475 = (_e467 * _e473);
        let _e476 = (_e468 * _e473);
        let _e487 = fma(fma(_e167.x, _e170.x, -0.04000000283122063), _e197, 0.04000000283122063);
        let _e488 = fma(fma(_e167.y, _e170.y, -0.04000000283122063), _e197, 0.04000000283122063);
        let _e489 = fma(fma(_e167.z, _e170.z, -0.04000000283122063), _e197, 0.04000000283122063);
        let _e495 = pow((1.0 - max(fma(_e230, _e476, fma(_e228, _e474, (_e229 * _e475))), 1.1920928955078125e-7)), 5.0);
        let _e504 = max(fma(_e351, _e465, fma(_e349, _e463, (_e350 * _e464))), 1.1920928955078125e-7);
        let _e508 = max(fma(_e351, _e230, fma(_e349, _e228, (_e350 * _e229))), 1.1920928955078125e-7);
        let _e512 = max(fma(_e351, _e476, fma(_e349, _e474, (_e350 * _e475))), 1.1920928955078125e-7);
        let _e513 = (_e462 * _e462);
        let _e516 = fma((_e512 * _e512), fma(_e462, _e462, -1.0), 1.0);
        let _e522 = fma(-(_e462), _e462, 1.0);
        let _e529 = fma(_e504, sqrt(fma((_e508 * _e508), _e522, _e513)), (_e508 * sqrt(fma((_e504 * _e504), _e522, _e513))));
        if (_e529 > 0.0) {
            phi_2046_ = (0.5 / _e529);
        } else {
            phi_2046_ = 0.0;
        }
        let _e533 = phi_2046_;
        let _e535 = ((vec3<f32>(_e487, _e488, _e489) + vec3<f32>(((1.0 - _e487) * _e495), ((1.0 - _e488) * _e495), ((1.0 - _e489) * _e495))) * ((_e513 / ((3.1415927410125732 * _e516) * _e516)) * _e533));
        let _e549 = fma(vec3<f32>(_e180.x, _e180.y, _e180.z), vec3<f32>(_e185, _e187, _e189), vec3<f32>(fma(((_e173 * 0.9599999785423279) * _e356), (_e166[0].x * fma(((1.0 - _e386) * (_e387 + 1.0)), pow((0.5 * (1.0 + fma(_e166[3].x, _e351, fma(_e166[1].x, _e349, (_e166[2].x * _e350))))), _e387), _e386)), ((_e535.x * ((_e166[0].x * 9.86960506439209) * _e456)) * _e504)), fma(((_e176 * 0.9599999785423279) * _e356), (_e166[0].y * fma(((1.0 - _e407) * (_e408 + 1.0)), pow((0.5 * (1.0 + fma(_e166[3].y, _e351, fma(_e166[1].y, _e349, (_e166[2].y * _e350))))), _e408), _e407)), ((_e535.y * ((_e166[0].y * 9.86960506439209) * _e456)) * _e504)), fma(((_e179 * 0.9599999785423279) * _e356), (_e166[0].z * fma(((1.0 - _e428) * (_e429 + 1.0)), pow((0.5 * (1.0 + fma(_e166[3].z, _e351, fma(_e166[1].z, _e349, (_e166[2].z * _e350))))), _e429), _e428)), ((_e535.z * ((_e166[0].z * 9.86960506439209) * _e456)) * _e504))));
        let _e552 = global.member.member_12;
        if ((_e552 & 2u) == 2u) {
            phi_1208_ = min(vec3<f32>(max(((_e549.x * fma(2.509999990463257, _e549.x, 0.029999999329447746)) / fma(_e549.x, fma(2.430000066757202, _e549.x, 0.5899999737739563), 0.14000000059604645)), 0.0), max(((_e549.y * fma(2.509999990463257, _e549.y, 0.029999999329447746)) / fma(_e549.y, fma(2.430000066757202, _e549.y, 0.5899999737739563), 0.14000000059604645)), 0.0), max(((_e549.z * fma(2.509999990463257, _e549.z, 0.029999999329447746)) / fma(_e549.z, fma(2.430000066757202, _e549.z, 0.5899999737739563), 0.14000000059604645)), 0.0)), vec3<f32>(1.0, 1.0, 1.0));
        } else {
            phi_1208_ = _e549;
        }
        let _e579 = phi_1208_;
        if ((_e552 & 4u) == 4u) {
            phi_1216_ = pow(_e579, vec3<f32>(0.45454543828964233, 0.45454543828964233, 0.45454543828964233));
        } else {
            phi_1216_ = _e579;
        }
        let _e584 = phi_1216_;
        global_21 = vec4<f32>(_e584.x, _e584.y, _e584.z, 1.0);
    }
    return;
}

@fragment 
fn single_view__fragment(@location(0) param: vec3<f32>, @location(1) param_1: vec3<f32>, @location(2) param_2: vec2<f32>, @location(3) param_3: vec2<f32>, @location(5) @interpolate(flat) param_4: u32, @builtin(front_facing) param_5: bool, @location(4) @interpolate(flat) param_6: u32) -> @location(0) vec4<f32> {
    global_2 = param;
    global_3 = param_1;
    global_4 = param_2;
    global_5 = param_3;
    global_6 = param_4;
    global_7 = param_5;
    global_23 = param_6;
    function();
    let _e15 = global_21;
    return _e15;
}
