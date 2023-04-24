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

@group(0) @binding(0) 
var<uniform> global: type_13;
@group(1) @binding(4) 
var<uniform> global_1: type_17;
var<private> global_2: vec3<f32>;
var<private> global_3: vec3<f32>;
var<private> global_4: vec2<f32>;
var<private> global_5: vec2<f32>;
var<private> global_6: u32;
var<private> global_7: i32;
var<private> global_8: bool;
@group(0) @binding(1) 
var global_9: sampler;
@group(0) @binding(3) 
var global_10: texture_3d<f32>;
@group(0) @binding(4) 
var global_11: texture_3d<f32>;
@group(0) @binding(5) 
var global_12: texture_3d<f32>;
@group(0) @binding(6) 
var global_13: texture_3d<f32>;
@group(0) @binding(7) 
var global_14: texture_2d<f32>;
@group(0) @binding(8) 
var global_15: texture_2d<f32>;
@group(0) @binding(9) 
var global_16: texture_2d<f32>;
@group(0) @binding(10) 
var global_17: texture_2d<f32>;
@group(1) @binding(5) 
var global_18: sampler;
@group(1) @binding(0) 
var global_19: texture_2d<f32>;
@group(1) @binding(2) 
var global_20: texture_2d<f32>;
@group(1) @binding(3) 
var global_21: texture_2d<f32>;
var<private> global_22: vec4<f32>;
@group(1) @binding(1) 
var global_23: texture_2d<f32>;
var<private> global_24: u32;

fn function() {
    var phi_287_: array<vec3<f32>,4u>;
    var phi_341_: vec3<f32>;
    var phi_364_: vec3<f32>;
    var phi_1414_: vec3<f32>;
    var phi_2048_: f32;
    var phi_441_: vec3<f32>;
    var phi_449_: vec3<f32>;

    let _e73 = global_2;
    let _e74 = global_3;
    let _e75 = global_4;
    let _e76 = global_5;
    let _e77 = global_6;
    let _e79 = global_7;
    let _e80 = global_8;
    if (_e77 == 0u) {
        let _e84 = global.member.member_13;
        let _e87 = global.member.member_14;
        let _e90 = global.member.member_15;
        let _e93 = global.member.member_16;
        let _e96 = global.member.member_17;
        let _e99 = global.member.member_18;
        let _e108 = (vec3<f32>((_e73.x - _e84), (_e73.y - _e87), (_e73.z - _e90)) / vec3<f32>(_e93, _e96, _e99));
        let _e109 = textureSampleLevel(global_10, global_9, _e108, 0.0);
        let _e114 = textureSampleLevel(global_11, global_9, _e108, 0.0);
        let _e122 = textureSampleLevel(global_12, global_9, _e108, 0.0);
        let _e130 = textureSampleLevel(global_13, global_9, _e108, 0.0);
        phi_287_ = array<vec3<f32>,4u>(vec3<f32>(_e109.x, _e109.y, _e109.z), vec3<f32>(fma(_e114.x, 2.007874011993408, -1.0078740119934082), fma(_e114.y, 2.007874011993408, -1.0078740119934082), fma(_e114.z, 2.007874011993408, -1.0078740119934082)), vec3<f32>(fma(_e122.x, 2.007874011993408, -1.0078740119934082), fma(_e122.y, 2.007874011993408, -1.0078740119934082), fma(_e122.z, 2.007874011993408, -1.0078740119934082)), vec3<f32>(fma(_e130.x, 2.007874011993408, -1.0078740119934082), fma(_e130.y, 2.007874011993408, -1.0078740119934082), fma(_e130.z, 2.007874011993408, -1.0078740119934082)));
    } else {
        let _e139 = textureSampleLevel(global_14, global_9, _e76, 0.0);
        let _e144 = textureSampleLevel(global_15, global_9, _e76, 0.0);
        let _e152 = textureSampleLevel(global_16, global_9, _e76, 0.0);
        let _e160 = textureSampleLevel(global_17, global_9, _e76, 0.0);
        phi_287_ = array<vec3<f32>,4u>(vec3<f32>(_e139.x, _e139.y, _e139.z), vec3<f32>(fma(_e144.x, 2.007874011993408, -1.0078740119934082), fma(_e144.y, 2.007874011993408, -1.0078740119934082), fma(_e144.z, 2.007874011993408, -1.0078740119934082)), vec3<f32>(fma(_e152.x, 2.007874011993408, -1.0078740119934082), fma(_e152.y, 2.007874011993408, -1.0078740119934082), fma(_e152.z, 2.007874011993408, -1.0078740119934082)), vec3<f32>(fma(_e160.x, 2.007874011993408, -1.0078740119934082), fma(_e160.y, 2.007874011993408, -1.0078740119934082), fma(_e160.z, 2.007874011993408, -1.0078740119934082)));
    }
    let _e170 = phi_287_;
    let _e171 = textureSample(global_19, global_18, _e75);
    let _e174 = global_1.member.member;
    let _e177 = (_e171.x * _e174.x);
    let _e180 = (_e171.y * _e174.y);
    let _e183 = (_e171.z * _e174.z);
    let _e184 = textureSample(global_21, global_18, _e75);
    let _e189 = global_1.member.member_3;
    let _e191 = global_1.member.member_4;
    let _e193 = global_1.member.member_5;
    let _e196 = textureSample(global_20, global_18, _e75);
    let _e200 = global_1.member.member_7;
    let _e201 = (_e196.z * _e200);
    let _e205 = global_1.member.member_8;
    let _e206 = vec3<f32>(_e177, _e180, _e183);
    let _e209 = global_1.member.member_10;
    if ((_e209 & 1u) == 1u) {
        let _e214 = global.member.member_12;
        if ((_e214 & 4u) == 4u) {
            phi_341_ = pow(_e206, vec3<f32>(0.45454543828964233, 0.45454543828964233, 0.45454543828964233));
        } else {
            phi_341_ = _e206;
        }
        let _e219 = phi_341_;
        global_22 = vec4<f32>(_e219.x, _e219.y, _e219.z, 1.0);
    } else {
        if (_e79 == 0) {
            let _e227 = global.member.member_6;
            let _e230 = global.member.member_7;
            let _e233 = global.member.member_8;
            phi_364_ = vec3<f32>(_e227, _e230, _e233);
        } else {
            let _e237 = global.member.member_9;
            let _e240 = global.member.member_10;
            let _e243 = global.member.member_11;
            phi_364_ = vec3<f32>(_e237, _e240, _e243);
        }
        let _e246 = phi_364_;
        let _e247 = (_e246 - _e73);
        let _e255 = (1.0 / sqrt(fma(_e247.z, _e247.z, fma(_e247.x, _e247.x, (_e247.y * _e247.y)))));
        let _e256 = (_e247.x * _e255);
        let _e257 = (_e247.y * _e255);
        let _e258 = (_e247.z * _e255);
        let _e262 = global_1.member.member_9;
        let _e270 = (1.0 / sqrt(fma(_e74.z, _e74.z, fma(_e74.x, _e74.x, (_e74.y * _e74.y)))));
        if (_e80 != true) {
            phi_1414_ = vec3<f32>(-((_e74.x * _e270)), -((_e74.y * _e270)), -((_e74.z * _e270)));
        } else {
            phi_1414_ = (_e74 * _e270);
        }
        let _e281 = phi_1414_;
        let _e282 = textureSample(global_23, global_18, _e75);
        let _e288 = fma(_e282.z, 2.007874011993408, -1.0078740119934082);
        let _e289 = (fma(_e282.x, 2.007874011993408, -1.0078740119934082) * _e262);
        let _e290 = (fma(_e282.y, 2.007874011993408, -1.0078740119934082) * _e262);
        let _e295 = (1.0 / sqrt(fma(_e288, _e288, fma(_e289, _e289, (_e290 * _e290)))));
        let _e296 = (_e289 * _e295);
        let _e297 = (_e290 * _e295);
        let _e298 = (_e288 * _e295);
        let _e299 = -(vec3<f32>(_e256, _e257, _e258));
        let _e300 = dpdx(_e299);
        let _e301 = dpdy(_e299);
        let _e302 = dpdx(_e75);
        let _e303 = dpdy(_e75);
        let _e310 = fma(_e301.y, _e281.z, -((_e281.y * _e301.z)));
        let _e315 = fma(_e301.z, _e281.x, -((_e281.z * _e301.x)));
        let _e318 = fma(_e301.x, _e281.y, -((_e281.x * _e301.y)));
        let _e332 = (_e303 * fma(_e281.y, _e300.z, -((_e300.y * _e281.z))));
        let _e335 = (_e303 * fma(_e281.z, _e300.x, -((_e300.z * _e281.x))));
        let _e338 = (_e303 * fma(_e281.x, _e300.y, -((_e300.x * _e281.y))));
        let _e341 = fma(_e310, _e302.x, _e332.x);
        let _e342 = fma(_e315, _e302.x, _e335.x);
        let _e343 = fma(_e318, _e302.x, _e338.x);
        let _e345 = fma(_e310, _e302.y, _e332.y);
        let _e346 = fma(_e315, _e302.y, _e335.y);
        let _e347 = fma(_e318, _e302.y, _e338.y);
        let _e356 = (1.0 / sqrt(max(fma(_e343, _e343, fma(_e341, _e341, (_e342 * _e342))), fma(_e347, _e347, fma(_e345, _e345, (_e346 * _e346))))));
        let _e369 = fma(_e281.x, _e298, fma((_e341 * _e356), _e296, ((_e345 * _e356) * _e297)));
        let _e370 = fma(_e281.y, _e298, fma((_e342 * _e356), _e296, ((_e346 * _e356) * _e297)));
        let _e371 = fma(_e281.z, _e298, fma((_e343 * _e356), _e296, ((_e347 * _e356) * _e297)));
        let _e376 = (1.0 / sqrt(fma(_e371, _e371, fma(_e369, _e369, (_e370 * _e370)))));
        let _e377 = (_e369 * _e376);
        let _e378 = (_e370 * _e376);
        let _e379 = (_e371 * _e376);
        let _e384 = fma(-(_e196.z), _e200, 1.0);
        let _e411 = sqrt(fma(_e170[3].x, _e170[3].x, fma(_e170[1].x, _e170[1].x, (_e170[2].x * _e170[2].x))));
        let _e414 = ((1.0 - _e411) / (1.0 + _e411));
        let _e415 = fma(2.0, _e411, 1.0);
        let _e432 = sqrt(fma(_e170[3].y, _e170[3].y, fma(_e170[1].y, _e170[1].y, (_e170[2].y * _e170[2].y))));
        let _e435 = ((1.0 - _e432) / (1.0 + _e432));
        let _e436 = fma(2.0, _e432, 1.0);
        let _e453 = sqrt(fma(_e170[3].z, _e170[3].z, fma(_e170[1].z, _e170[1].z, (_e170[2].z * _e170[2].z))));
        let _e456 = ((1.0 - _e453) / (1.0 + _e453));
        let _e457 = fma(2.0, _e453, 1.0);
        let _e478 = (((_e170[1].x + _e170[1].y) + _e170[1].z) * 0.3333333432674408);
        let _e479 = (((_e170[2].x + _e170[2].y) + _e170[2].z) * 0.3333333432674408);
        let _e480 = (((_e170[3].x + _e170[3].y) + _e170[3].z) * 0.3333333432674408);
        let _e484 = sqrt(fma(_e480, _e480, fma(_e478, _e478, (_e479 * _e479))));
        let _e489 = fma(-(fma(-(_e196.y), _e205, 1.0)), sqrt(_e484), 1.0);
        let _e490 = (_e489 * _e489);
        let _e491 = (_e478 / _e484);
        let _e492 = (_e479 / _e484);
        let _e493 = (_e480 / _e484);
        let _e494 = fma(_e247.x, _e255, _e491);
        let _e495 = fma(_e247.y, _e255, _e492);
        let _e496 = fma(_e247.z, _e255, _e493);
        let _e501 = (1.0 / sqrt(fma(_e496, _e496, fma(_e494, _e494, (_e495 * _e495)))));
        let _e502 = (_e494 * _e501);
        let _e503 = (_e495 * _e501);
        let _e504 = (_e496 * _e501);
        let _e515 = fma(fma(_e171.x, _e174.x, -0.04000000283122063), _e201, 0.04000000283122063);
        let _e516 = fma(fma(_e171.y, _e174.y, -0.04000000283122063), _e201, 0.04000000283122063);
        let _e517 = fma(fma(_e171.z, _e174.z, -0.04000000283122063), _e201, 0.04000000283122063);
        let _e523 = pow((1.0 - max(fma(_e258, _e504, fma(_e256, _e502, (_e257 * _e503))), 1.1920928955078125e-7)), 5.0);
        let _e532 = max(fma(_e379, _e493, fma(_e377, _e491, (_e378 * _e492))), 1.1920928955078125e-7);
        let _e536 = max(fma(_e379, _e258, fma(_e377, _e256, (_e378 * _e257))), 1.1920928955078125e-7);
        let _e540 = max(fma(_e379, _e504, fma(_e377, _e502, (_e378 * _e503))), 1.1920928955078125e-7);
        let _e541 = (_e490 * _e490);
        let _e544 = fma((_e540 * _e540), fma(_e490, _e490, -1.0), 1.0);
        let _e550 = fma(-(_e490), _e490, 1.0);
        let _e557 = fma(_e532, sqrt(fma((_e536 * _e536), _e550, _e541)), (_e536 * sqrt(fma((_e532 * _e532), _e550, _e541))));
        if (_e557 > 0.0) {
            phi_2048_ = (0.5 / _e557);
        } else {
            phi_2048_ = 0.0;
        }
        let _e561 = phi_2048_;
        let _e563 = ((vec3<f32>(_e515, _e516, _e517) + vec3<f32>(((1.0 - _e515) * _e523), ((1.0 - _e516) * _e523), ((1.0 - _e517) * _e523))) * ((_e541 / ((3.1415927410125732 * _e544) * _e544)) * _e561));
        let _e577 = fma(vec3<f32>(_e184.x, _e184.y, _e184.z), vec3<f32>(_e189, _e191, _e193), vec3<f32>(fma(((_e177 * 0.9599999785423279) * _e384), (_e170[0].x * fma(((1.0 - _e414) * (_e415 + 1.0)), pow((0.5 * (1.0 + fma(_e170[3].x, _e379, fma(_e170[1].x, _e377, (_e170[2].x * _e378))))), _e415), _e414)), ((_e563.x * ((_e170[0].x * 9.86960506439209) * _e484)) * _e532)), fma(((_e180 * 0.9599999785423279) * _e384), (_e170[0].y * fma(((1.0 - _e435) * (_e436 + 1.0)), pow((0.5 * (1.0 + fma(_e170[3].y, _e379, fma(_e170[1].y, _e377, (_e170[2].y * _e378))))), _e436), _e435)), ((_e563.y * ((_e170[0].y * 9.86960506439209) * _e484)) * _e532)), fma(((_e183 * 0.9599999785423279) * _e384), (_e170[0].z * fma(((1.0 - _e456) * (_e457 + 1.0)), pow((0.5 * (1.0 + fma(_e170[3].z, _e379, fma(_e170[1].z, _e377, (_e170[2].z * _e378))))), _e457), _e456)), ((_e563.z * ((_e170[0].z * 9.86960506439209) * _e484)) * _e532))));
        let _e580 = global.member.member_12;
        if ((_e580 & 2u) == 2u) {
            phi_441_ = min(vec3<f32>(max(((_e577.x * fma(2.509999990463257, _e577.x, 0.029999999329447746)) / fma(_e577.x, fma(2.430000066757202, _e577.x, 0.5899999737739563), 0.14000000059604645)), 0.0), max(((_e577.y * fma(2.509999990463257, _e577.y, 0.029999999329447746)) / fma(_e577.y, fma(2.430000066757202, _e577.y, 0.5899999737739563), 0.14000000059604645)), 0.0), max(((_e577.z * fma(2.509999990463257, _e577.z, 0.029999999329447746)) / fma(_e577.z, fma(2.430000066757202, _e577.z, 0.5899999737739563), 0.14000000059604645)), 0.0)), vec3<f32>(1.0, 1.0, 1.0));
        } else {
            phi_441_ = _e577;
        }
        let _e607 = phi_441_;
        if ((_e580 & 4u) == 4u) {
            phi_449_ = pow(_e607, vec3<f32>(0.45454543828964233, 0.45454543828964233, 0.45454543828964233));
        } else {
            phi_449_ = _e607;
        }
        let _e612 = phi_449_;
        global_22 = vec4<f32>(_e612.x, _e612.y, _e612.z, 1.0);
    }
    return;
}

@fragment 
fn fragment(@location(0) param: vec3<f32>, @location(1) param_1: vec3<f32>, @location(2) param_2: vec2<f32>, @location(3) param_3: vec2<f32>, @location(5) param_4: u32, @builtin(view_index) param_5: i32, @builtin(front_facing) param_6: bool, @location(4) param_7: u32) -> @location(0) vec4<f32> {
    global_2 = param;
    global_3 = param_1;
    global_4 = param_2;
    global_5 = param_3;
    global_6 = param_4;
    global_7 = param_5;
    global_8 = param_6;
    global_24 = param_7;
    function();
    let _e17 = global_22;
    return _e17;
}
