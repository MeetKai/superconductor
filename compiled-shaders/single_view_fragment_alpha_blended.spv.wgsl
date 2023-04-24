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
    let _e207 = global_1.member.member_10;
    if ((_e207 & 1u) == 1u) {
        let _e593 = global.member.member_12;
        if ((_e593 & 4u) == 4u) {
            phi_1108_ = pow(_e204, vec3<f32>(0.45454543828964233, 0.45454543828964233, 0.45454543828964233));
        } else {
            phi_1108_ = _e204;
        }
        let _e598 = phi_1108_;
        global_21 = vec4<f32>(_e598.x, _e598.y, _e598.z, _e173.w);
    } else {
        let _e212 = global.member.member_6;
        let _e215 = global.member.member_7;
        let _e218 = global.member.member_8;
        let _e220 = (_e212 - _e70.x);
        let _e222 = (_e215 - _e70.y);
        let _e224 = (_e218 - _e70.z);
        let _e229 = (1.0 / sqrt(fma(_e224, _e224, fma(_e220, _e220, (_e222 * _e222)))));
        let _e230 = (_e220 * _e229);
        let _e231 = (_e222 * _e229);
        let _e232 = (_e224 * _e229);
        let _e236 = global_1.member.member_9;
        let _e244 = (1.0 / sqrt(fma(_e71.z, _e71.z, fma(_e71.x, _e71.x, (_e71.y * _e71.y)))));
        if (_e76 != true) {
            phi_1412_ = vec3<f32>(-((_e71.x * _e244)), -((_e71.y * _e244)), -((_e71.z * _e244)));
        } else {
            phi_1412_ = (_e71 * _e244);
        }
        let _e255 = phi_1412_;
        let _e256 = textureSample(global_22, global_17, _e72);
        let _e262 = fma(_e256.z, 2.007874011993408, -1.0078740119934082);
        let _e263 = (fma(_e256.x, 2.007874011993408, -1.0078740119934082) * _e236);
        let _e264 = (fma(_e256.y, 2.007874011993408, -1.0078740119934082) * _e236);
        let _e269 = (1.0 / sqrt(fma(_e262, _e262, fma(_e263, _e263, (_e264 * _e264)))));
        let _e270 = (_e263 * _e269);
        let _e271 = (_e264 * _e269);
        let _e272 = (_e262 * _e269);
        let _e273 = -(vec3<f32>(_e230, _e231, _e232));
        let _e274 = dpdx(_e273);
        let _e275 = dpdy(_e273);
        let _e276 = dpdx(_e72);
        let _e277 = dpdy(_e72);
        let _e284 = fma(_e275.y, _e255.z, -((_e255.y * _e275.z)));
        let _e289 = fma(_e275.z, _e255.x, -((_e255.z * _e275.x)));
        let _e292 = fma(_e275.x, _e255.y, -((_e255.x * _e275.y)));
        let _e306 = (_e277 * fma(_e255.y, _e274.z, -((_e274.y * _e255.z))));
        let _e309 = (_e277 * fma(_e255.z, _e274.x, -((_e274.z * _e255.x))));
        let _e312 = (_e277 * fma(_e255.x, _e274.y, -((_e274.x * _e255.y))));
        let _e315 = fma(_e284, _e276.x, _e306.x);
        let _e316 = fma(_e289, _e276.x, _e309.x);
        let _e317 = fma(_e292, _e276.x, _e312.x);
        let _e319 = fma(_e284, _e276.y, _e306.y);
        let _e320 = fma(_e289, _e276.y, _e309.y);
        let _e321 = fma(_e292, _e276.y, _e312.y);
        let _e330 = (1.0 / sqrt(max(fma(_e317, _e317, fma(_e315, _e315, (_e316 * _e316))), fma(_e321, _e321, fma(_e319, _e319, (_e320 * _e320))))));
        let _e343 = fma(_e255.x, _e272, fma((_e315 * _e330), _e270, ((_e319 * _e330) * _e271)));
        let _e344 = fma(_e255.y, _e272, fma((_e316 * _e330), _e270, ((_e320 * _e330) * _e271)));
        let _e345 = fma(_e255.z, _e272, fma((_e317 * _e330), _e270, ((_e321 * _e330) * _e271)));
        let _e350 = (1.0 / sqrt(fma(_e345, _e345, fma(_e343, _e343, (_e344 * _e344)))));
        let _e351 = (_e343 * _e350);
        let _e352 = (_e344 * _e350);
        let _e353 = (_e345 * _e350);
        let _e358 = fma(-(_e194.z), _e198, 1.0);
        let _e385 = sqrt(fma(_e166[3].x, _e166[3].x, fma(_e166[1].x, _e166[1].x, (_e166[2].x * _e166[2].x))));
        let _e388 = ((1.0 - _e385) / (1.0 + _e385));
        let _e389 = fma(2.0, _e385, 1.0);
        let _e406 = sqrt(fma(_e166[3].y, _e166[3].y, fma(_e166[1].y, _e166[1].y, (_e166[2].y * _e166[2].y))));
        let _e409 = ((1.0 - _e406) / (1.0 + _e406));
        let _e410 = fma(2.0, _e406, 1.0);
        let _e427 = sqrt(fma(_e166[3].z, _e166[3].z, fma(_e166[1].z, _e166[1].z, (_e166[2].z * _e166[2].z))));
        let _e430 = ((1.0 - _e427) / (1.0 + _e427));
        let _e431 = fma(2.0, _e427, 1.0);
        let _e452 = (((_e166[1].x + _e166[1].y) + _e166[1].z) * 0.3333333432674408);
        let _e453 = (((_e166[2].x + _e166[2].y) + _e166[2].z) * 0.3333333432674408);
        let _e454 = (((_e166[3].x + _e166[3].y) + _e166[3].z) * 0.3333333432674408);
        let _e458 = sqrt(fma(_e454, _e454, fma(_e452, _e452, (_e453 * _e453))));
        let _e463 = fma(-(fma(-(_e194.y), _e203, 1.0)), sqrt(_e458), 1.0);
        let _e464 = (_e463 * _e463);
        let _e465 = (_e452 / _e458);
        let _e466 = (_e453 / _e458);
        let _e467 = (_e454 / _e458);
        let _e468 = fma(_e220, _e229, _e465);
        let _e469 = fma(_e222, _e229, _e466);
        let _e470 = fma(_e224, _e229, _e467);
        let _e475 = (1.0 / sqrt(fma(_e470, _e470, fma(_e468, _e468, (_e469 * _e469)))));
        let _e476 = (_e468 * _e475);
        let _e477 = (_e469 * _e475);
        let _e478 = (_e470 * _e475);
        let _e489 = fma(fma(_e167.x, _e170.x, -0.04000000283122063), _e199, 0.04000000283122063);
        let _e490 = fma(fma(_e167.y, _e170.y, -0.04000000283122063), _e199, 0.04000000283122063);
        let _e491 = fma(fma(_e167.z, _e170.z, -0.04000000283122063), _e199, 0.04000000283122063);
        let _e497 = pow((1.0 - max(fma(_e232, _e478, fma(_e230, _e476, (_e231 * _e477))), 1.1920928955078125e-7)), 5.0);
        let _e506 = max(fma(_e353, _e467, fma(_e351, _e465, (_e352 * _e466))), 1.1920928955078125e-7);
        let _e510 = max(fma(_e353, _e232, fma(_e351, _e230, (_e352 * _e231))), 1.1920928955078125e-7);
        let _e514 = max(fma(_e353, _e478, fma(_e351, _e476, (_e352 * _e477))), 1.1920928955078125e-7);
        let _e515 = (_e464 * _e464);
        let _e518 = fma((_e514 * _e514), fma(_e464, _e464, -1.0), 1.0);
        let _e524 = fma(-(_e464), _e464, 1.0);
        let _e531 = fma(_e506, sqrt(fma((_e510 * _e510), _e524, _e515)), (_e510 * sqrt(fma((_e506 * _e506), _e524, _e515))));
        if (_e531 > 0.0) {
            phi_2046_ = (0.5 / _e531);
        } else {
            phi_2046_ = 0.0;
        }
        let _e535 = phi_2046_;
        let _e537 = ((vec3<f32>(_e489, _e490, _e491) + vec3<f32>(((1.0 - _e489) * _e497), ((1.0 - _e490) * _e497), ((1.0 - _e491) * _e497))) * ((_e515 / ((3.1415927410125732 * _e518) * _e518)) * _e535));
        let _e551 = fma(vec3<f32>(_e182.x, _e182.y, _e182.z), vec3<f32>(_e187, _e189, _e191), vec3<f32>(fma(((_e173.x * 0.9599999785423279) * _e358), (_e166[0].x * fma(((1.0 - _e388) * (_e389 + 1.0)), pow((0.5 * (1.0 + fma(_e166[3].x, _e353, fma(_e166[1].x, _e351, (_e166[2].x * _e352))))), _e389), _e388)), ((_e537.x * ((_e166[0].x * 9.86960506439209) * _e458)) * _e506)), fma(((_e173.y * 0.9599999785423279) * _e358), (_e166[0].y * fma(((1.0 - _e409) * (_e410 + 1.0)), pow((0.5 * (1.0 + fma(_e166[3].y, _e353, fma(_e166[1].y, _e351, (_e166[2].y * _e352))))), _e410), _e409)), ((_e537.y * ((_e166[0].y * 9.86960506439209) * _e458)) * _e506)), fma(((_e173.z * 0.9599999785423279) * _e358), (_e166[0].z * fma(((1.0 - _e430) * (_e431 + 1.0)), pow((0.5 * (1.0 + fma(_e166[3].z, _e353, fma(_e166[1].z, _e351, (_e166[2].z * _e352))))), _e431), _e430)), ((_e537.z * ((_e166[0].z * 9.86960506439209) * _e458)) * _e506))));
        let _e554 = global.member.member_12;
        if ((_e554 & 2u) == 2u) {
            phi_1208_ = min(vec3<f32>(max(((_e551.x * fma(2.509999990463257, _e551.x, 0.029999999329447746)) / fma(_e551.x, fma(2.430000066757202, _e551.x, 0.5899999737739563), 0.14000000059604645)), 0.0), max(((_e551.y * fma(2.509999990463257, _e551.y, 0.029999999329447746)) / fma(_e551.y, fma(2.430000066757202, _e551.y, 0.5899999737739563), 0.14000000059604645)), 0.0), max(((_e551.z * fma(2.509999990463257, _e551.z, 0.029999999329447746)) / fma(_e551.z, fma(2.430000066757202, _e551.z, 0.5899999737739563), 0.14000000059604645)), 0.0)), vec3<f32>(1.0, 1.0, 1.0));
        } else {
            phi_1208_ = _e551;
        }
        let _e581 = phi_1208_;
        if ((_e554 & 4u) == 4u) {
            phi_1216_ = pow(_e581, vec3<f32>(0.45454543828964233, 0.45454543828964233, 0.45454543828964233));
        } else {
            phi_1216_ = _e581;
        }
        let _e586 = phi_1216_;
        global_21 = vec4<f32>(_e586.x, _e586.y, _e586.z, _e173.w);
    }
    return;
}

@fragment 
fn single_view__fragment_alpha_blended(@location(0) param: vec3<f32>, @location(1) param_1: vec3<f32>, @location(2) param_2: vec2<f32>, @location(3) param_3: vec2<f32>, @location(5) @interpolate(flat) param_4: u32, @builtin(front_facing) param_5: bool, @location(4) @interpolate(flat) param_6: u32) -> @location(0) vec4<f32> {
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
