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
    var phi_1057_: array<vec3<f32>,4u>;
    var phi_1401_: vec3<f32>;
    var phi_2056_: f32;
    var phi_1216_: vec3<f32>;
    var phi_1224_: vec3<f32>;
    var phi_1160_: vec3<f32>;

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
        phi_1057_ = array<vec3<f32>,4u>(vec3<f32>(_e135.x, _e135.y, _e135.z), vec3<f32>(fma(_e140.x, 2.007874011993408, -1.0078740119934082), fma(_e140.y, 2.007874011993408, -1.0078740119934082), fma(_e140.z, 2.007874011993408, -1.0078740119934082)), vec3<f32>(fma(_e148.x, 2.007874011993408, -1.0078740119934082), fma(_e148.y, 2.007874011993408, -1.0078740119934082), fma(_e148.z, 2.007874011993408, -1.0078740119934082)), vec3<f32>(fma(_e156.x, 2.007874011993408, -1.0078740119934082), fma(_e156.y, 2.007874011993408, -1.0078740119934082), fma(_e156.z, 2.007874011993408, -1.0078740119934082)));
    } else {
        let _e78 = textureSampleLevel(global_13, global_8, _e73, 0.0);
        let _e83 = textureSampleLevel(global_14, global_8, _e73, 0.0);
        let _e91 = textureSampleLevel(global_15, global_8, _e73, 0.0);
        let _e99 = textureSampleLevel(global_16, global_8, _e73, 0.0);
        phi_1057_ = array<vec3<f32>,4u>(vec3<f32>(_e78.x, _e78.y, _e78.z), vec3<f32>(fma(_e83.x, 2.007874011993408, -1.0078740119934082), fma(_e83.y, 2.007874011993408, -1.0078740119934082), fma(_e83.z, 2.007874011993408, -1.0078740119934082)), vec3<f32>(fma(_e91.x, 2.007874011993408, -1.0078740119934082), fma(_e91.y, 2.007874011993408, -1.0078740119934082), fma(_e91.z, 2.007874011993408, -1.0078740119934082)), vec3<f32>(fma(_e99.x, 2.007874011993408, -1.0078740119934082), fma(_e99.y, 2.007874011993408, -1.0078740119934082), fma(_e99.z, 2.007874011993408, -1.0078740119934082)));
    }
    let _e166 = phi_1057_;
    let _e167 = textureSample(global_18, global_17, _e72);
    let _e170 = global_1.member.member;
    let _e173 = (_e167 * _e170);
    let _e182 = textureSample(global_20, global_17, _e72);
    let _e187 = global_1.member.member_3;
    let _e189 = global_1.member.member_4;
    let _e191 = global_1.member.member_5;
    let _e194 = textureSample(global_19, global_17, _e72);
    let _e198 = global_1.member.member_7;
    let _e199 = (_e194.z * _e198);
    let _e203 = global_1.member.member_8;
    let _e204 = vec3<f32>(_e173.x, _e173.y, _e173.z);
    let _e207 = global.member.member_6;
    let _e210 = global.member.member_7;
    let _e213 = global.member.member_8;
    let _e215 = (_e207 - _e70.x);
    let _e217 = (_e210 - _e70.y);
    let _e219 = (_e213 - _e70.z);
    let _e224 = (1.0 / sqrt(fma(_e219, _e219, fma(_e215, _e215, (_e217 * _e217)))));
    let _e225 = (_e215 * _e224);
    let _e226 = (_e217 * _e224);
    let _e227 = (_e219 * _e224);
    let _e231 = global_1.member.member_9;
    let _e239 = (1.0 / sqrt(fma(_e71.z, _e71.z, fma(_e71.x, _e71.x, (_e71.y * _e71.y)))));
    if (_e76 != true) {
        phi_1401_ = vec3<f32>(-((_e71.x * _e239)), -((_e71.y * _e239)), -((_e71.z * _e239)));
    } else {
        phi_1401_ = (_e71 * _e239);
    }
    let _e250 = phi_1401_;
    let _e251 = textureSample(global_22, global_17, _e72);
    let _e257 = fma(_e251.z, 2.007874011993408, -1.0078740119934082);
    let _e258 = (fma(_e251.x, 2.007874011993408, -1.0078740119934082) * _e231);
    let _e259 = (fma(_e251.y, 2.007874011993408, -1.0078740119934082) * _e231);
    let _e264 = (1.0 / sqrt(fma(_e257, _e257, fma(_e258, _e258, (_e259 * _e259)))));
    let _e265 = (_e258 * _e264);
    let _e266 = (_e259 * _e264);
    let _e267 = (_e257 * _e264);
    let _e268 = -(vec3<f32>(_e225, _e226, _e227));
    let _e269 = dpdx(_e268);
    let _e270 = dpdy(_e268);
    let _e271 = dpdx(_e72);
    let _e272 = dpdy(_e72);
    let _e279 = fma(_e270.y, _e250.z, -((_e250.y * _e270.z)));
    let _e284 = fma(_e270.z, _e250.x, -((_e250.z * _e270.x)));
    let _e287 = fma(_e270.x, _e250.y, -((_e250.x * _e270.y)));
    let _e301 = (_e272 * fma(_e250.y, _e269.z, -((_e269.y * _e250.z))));
    let _e304 = (_e272 * fma(_e250.z, _e269.x, -((_e269.z * _e250.x))));
    let _e307 = (_e272 * fma(_e250.x, _e269.y, -((_e269.x * _e250.y))));
    let _e310 = fma(_e279, _e271.x, _e301.x);
    let _e311 = fma(_e284, _e271.x, _e304.x);
    let _e312 = fma(_e287, _e271.x, _e307.x);
    let _e314 = fma(_e279, _e271.y, _e301.y);
    let _e315 = fma(_e284, _e271.y, _e304.y);
    let _e316 = fma(_e287, _e271.y, _e307.y);
    let _e325 = (1.0 / sqrt(max(fma(_e312, _e312, fma(_e310, _e310, (_e311 * _e311))), fma(_e316, _e316, fma(_e314, _e314, (_e315 * _e315))))));
    let _e338 = fma(_e250.x, _e267, fma((_e310 * _e325), _e265, ((_e314 * _e325) * _e266)));
    let _e339 = fma(_e250.y, _e267, fma((_e311 * _e325), _e265, ((_e315 * _e325) * _e266)));
    let _e340 = fma(_e250.z, _e267, fma((_e312 * _e325), _e265, ((_e316 * _e325) * _e266)));
    let _e345 = (1.0 / sqrt(fma(_e340, _e340, fma(_e338, _e338, (_e339 * _e339)))));
    let _e346 = (_e338 * _e345);
    let _e347 = (_e339 * _e345);
    let _e348 = (_e340 * _e345);
    if (_e173.w < 0.5) {
        discard;
    } else {
        let _e352 = global_1.member.member_10;
        if ((_e352 & 1u) == 1u) {
            let _e594 = global.member.member_12;
            if ((_e594 & 4u) == 4u) {
                phi_1160_ = pow(_e204, vec3<f32>(0.45454543828964233, 0.45454543828964233, 0.45454543828964233));
            } else {
                phi_1160_ = _e204;
            }
            let _e599 = phi_1160_;
            global_21 = vec4<f32>(_e599.x, _e599.y, _e599.z, 1.0);
        } else {
            let _e359 = fma(-(_e194.z), _e198, 1.0);
            let _e386 = sqrt(fma(_e166[3].x, _e166[3].x, fma(_e166[1].x, _e166[1].x, (_e166[2].x * _e166[2].x))));
            let _e389 = ((1.0 - _e386) / (1.0 + _e386));
            let _e390 = fma(2.0, _e386, 1.0);
            let _e407 = sqrt(fma(_e166[3].y, _e166[3].y, fma(_e166[1].y, _e166[1].y, (_e166[2].y * _e166[2].y))));
            let _e410 = ((1.0 - _e407) / (1.0 + _e407));
            let _e411 = fma(2.0, _e407, 1.0);
            let _e428 = sqrt(fma(_e166[3].z, _e166[3].z, fma(_e166[1].z, _e166[1].z, (_e166[2].z * _e166[2].z))));
            let _e431 = ((1.0 - _e428) / (1.0 + _e428));
            let _e432 = fma(2.0, _e428, 1.0);
            let _e453 = (((_e166[1].x + _e166[1].y) + _e166[1].z) * 0.3333333432674408);
            let _e454 = (((_e166[2].x + _e166[2].y) + _e166[2].z) * 0.3333333432674408);
            let _e455 = (((_e166[3].x + _e166[3].y) + _e166[3].z) * 0.3333333432674408);
            let _e459 = sqrt(fma(_e455, _e455, fma(_e453, _e453, (_e454 * _e454))));
            let _e464 = fma(-(fma(-(_e194.y), _e203, 1.0)), sqrt(_e459), 1.0);
            let _e465 = (_e464 * _e464);
            let _e466 = (_e453 / _e459);
            let _e467 = (_e454 / _e459);
            let _e468 = (_e455 / _e459);
            let _e469 = fma(_e215, _e224, _e466);
            let _e470 = fma(_e217, _e224, _e467);
            let _e471 = fma(_e219, _e224, _e468);
            let _e476 = (1.0 / sqrt(fma(_e471, _e471, fma(_e469, _e469, (_e470 * _e470)))));
            let _e477 = (_e469 * _e476);
            let _e478 = (_e470 * _e476);
            let _e479 = (_e471 * _e476);
            let _e490 = fma(fma(_e167.x, _e170.x, -0.04000000283122063), _e199, 0.04000000283122063);
            let _e491 = fma(fma(_e167.y, _e170.y, -0.04000000283122063), _e199, 0.04000000283122063);
            let _e492 = fma(fma(_e167.z, _e170.z, -0.04000000283122063), _e199, 0.04000000283122063);
            let _e498 = pow((1.0 - max(fma(_e227, _e479, fma(_e225, _e477, (_e226 * _e478))), 1.1920928955078125e-7)), 5.0);
            let _e507 = max(fma(_e348, _e468, fma(_e346, _e466, (_e347 * _e467))), 1.1920928955078125e-7);
            let _e511 = max(fma(_e348, _e227, fma(_e346, _e225, (_e347 * _e226))), 1.1920928955078125e-7);
            let _e515 = max(fma(_e348, _e479, fma(_e346, _e477, (_e347 * _e478))), 1.1920928955078125e-7);
            let _e516 = (_e465 * _e465);
            let _e519 = fma((_e515 * _e515), fma(_e465, _e465, -1.0), 1.0);
            let _e525 = fma(-(_e465), _e465, 1.0);
            let _e532 = fma(_e507, sqrt(fma((_e511 * _e511), _e525, _e516)), (_e511 * sqrt(fma((_e507 * _e507), _e525, _e516))));
            if (_e532 > 0.0) {
                phi_2056_ = (0.5 / _e532);
            } else {
                phi_2056_ = 0.0;
            }
            let _e536 = phi_2056_;
            let _e538 = ((vec3<f32>(_e490, _e491, _e492) + vec3<f32>(((1.0 - _e490) * _e498), ((1.0 - _e491) * _e498), ((1.0 - _e492) * _e498))) * ((_e516 / ((3.1415927410125732 * _e519) * _e519)) * _e536));
            let _e552 = fma(vec3<f32>(_e182.x, _e182.y, _e182.z), vec3<f32>(_e187, _e189, _e191), vec3<f32>(fma(((_e173.x * 0.9599999785423279) * _e359), (_e166[0].x * fma(((1.0 - _e389) * (_e390 + 1.0)), pow((0.5 * (1.0 + fma(_e166[3].x, _e348, fma(_e166[1].x, _e346, (_e166[2].x * _e347))))), _e390), _e389)), ((_e538.x * ((_e166[0].x * 9.86960506439209) * _e459)) * _e507)), fma(((_e173.y * 0.9599999785423279) * _e359), (_e166[0].y * fma(((1.0 - _e410) * (_e411 + 1.0)), pow((0.5 * (1.0 + fma(_e166[3].y, _e348, fma(_e166[1].y, _e346, (_e166[2].y * _e347))))), _e411), _e410)), ((_e538.y * ((_e166[0].y * 9.86960506439209) * _e459)) * _e507)), fma(((_e173.z * 0.9599999785423279) * _e359), (_e166[0].z * fma(((1.0 - _e431) * (_e432 + 1.0)), pow((0.5 * (1.0 + fma(_e166[3].z, _e348, fma(_e166[1].z, _e346, (_e166[2].z * _e347))))), _e432), _e431)), ((_e538.z * ((_e166[0].z * 9.86960506439209) * _e459)) * _e507))));
            let _e555 = global.member.member_12;
            if ((_e555 & 2u) == 2u) {
                phi_1216_ = min(vec3<f32>(max(((_e552.x * fma(2.509999990463257, _e552.x, 0.029999999329447746)) / fma(_e552.x, fma(2.430000066757202, _e552.x, 0.5899999737739563), 0.14000000059604645)), 0.0), max(((_e552.y * fma(2.509999990463257, _e552.y, 0.029999999329447746)) / fma(_e552.y, fma(2.430000066757202, _e552.y, 0.5899999737739563), 0.14000000059604645)), 0.0), max(((_e552.z * fma(2.509999990463257, _e552.z, 0.029999999329447746)) / fma(_e552.z, fma(2.430000066757202, _e552.z, 0.5899999737739563), 0.14000000059604645)), 0.0)), vec3<f32>(1.0, 1.0, 1.0));
            } else {
                phi_1216_ = _e552;
            }
            let _e582 = phi_1216_;
            if ((_e555 & 4u) == 4u) {
                phi_1224_ = pow(_e582, vec3<f32>(0.45454543828964233, 0.45454543828964233, 0.45454543828964233));
            } else {
                phi_1224_ = _e582;
            }
            let _e587 = phi_1224_;
            global_21 = vec4<f32>(_e587.x, _e587.y, _e587.z, 1.0);
        }
    }
    return;
}

@fragment 
fn single_view__fragment_alpha_clipped(@location(0) param: vec3<f32>, @location(1) param_1: vec3<f32>, @location(2) param_2: vec2<f32>, @location(3) param_3: vec2<f32>, @location(5) @interpolate(flat) param_4: u32, @builtin(front_facing) param_5: bool, @location(4) @interpolate(flat) param_6: u32) -> @location(0) vec4<f32> {
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
