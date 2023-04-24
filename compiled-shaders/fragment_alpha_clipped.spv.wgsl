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
    var phi_1058_: array<vec3<f32>,4u>;
    var phi_1116_: vec3<f32>;
    var phi_1403_: vec3<f32>;
    var phi_1161_: vec3<f32>;
    var phi_2058_: f32;
    var phi_1217_: vec3<f32>;
    var phi_1225_: vec3<f32>;

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
        phi_1058_ = array<vec3<f32>,4u>(vec3<f32>(_e109.x, _e109.y, _e109.z), vec3<f32>(fma(_e114.x, 2.007874011993408, -1.0078740119934082), fma(_e114.y, 2.007874011993408, -1.0078740119934082), fma(_e114.z, 2.007874011993408, -1.0078740119934082)), vec3<f32>(fma(_e122.x, 2.007874011993408, -1.0078740119934082), fma(_e122.y, 2.007874011993408, -1.0078740119934082), fma(_e122.z, 2.007874011993408, -1.0078740119934082)), vec3<f32>(fma(_e130.x, 2.007874011993408, -1.0078740119934082), fma(_e130.y, 2.007874011993408, -1.0078740119934082), fma(_e130.z, 2.007874011993408, -1.0078740119934082)));
    } else {
        let _e139 = textureSampleLevel(global_14, global_9, _e76, 0.0);
        let _e144 = textureSampleLevel(global_15, global_9, _e76, 0.0);
        let _e152 = textureSampleLevel(global_16, global_9, _e76, 0.0);
        let _e160 = textureSampleLevel(global_17, global_9, _e76, 0.0);
        phi_1058_ = array<vec3<f32>,4u>(vec3<f32>(_e139.x, _e139.y, _e139.z), vec3<f32>(fma(_e144.x, 2.007874011993408, -1.0078740119934082), fma(_e144.y, 2.007874011993408, -1.0078740119934082), fma(_e144.z, 2.007874011993408, -1.0078740119934082)), vec3<f32>(fma(_e152.x, 2.007874011993408, -1.0078740119934082), fma(_e152.y, 2.007874011993408, -1.0078740119934082), fma(_e152.z, 2.007874011993408, -1.0078740119934082)), vec3<f32>(fma(_e160.x, 2.007874011993408, -1.0078740119934082), fma(_e160.y, 2.007874011993408, -1.0078740119934082), fma(_e160.z, 2.007874011993408, -1.0078740119934082)));
    }
    let _e170 = phi_1058_;
    let _e171 = textureSample(global_19, global_18, _e75);
    let _e174 = global_1.member.member;
    let _e177 = (_e171 * _e174);
    let _e186 = textureSample(global_21, global_18, _e75);
    let _e191 = global_1.member.member_3;
    let _e193 = global_1.member.member_4;
    let _e195 = global_1.member.member_5;
    let _e198 = textureSample(global_20, global_18, _e75);
    let _e202 = global_1.member.member_7;
    let _e203 = (_e198.z * _e202);
    let _e207 = global_1.member.member_8;
    let _e208 = vec3<f32>(_e177.x, _e177.y, _e177.z);
    if (_e79 == 0) {
        let _e212 = global.member.member_6;
        let _e215 = global.member.member_7;
        let _e218 = global.member.member_8;
        phi_1116_ = vec3<f32>(_e212, _e215, _e218);
    } else {
        let _e222 = global.member.member_9;
        let _e225 = global.member.member_10;
        let _e228 = global.member.member_11;
        phi_1116_ = vec3<f32>(_e222, _e225, _e228);
    }
    let _e231 = phi_1116_;
    let _e232 = (_e231 - _e73);
    let _e240 = (1.0 / sqrt(fma(_e232.z, _e232.z, fma(_e232.x, _e232.x, (_e232.y * _e232.y)))));
    let _e241 = (_e232.x * _e240);
    let _e242 = (_e232.y * _e240);
    let _e243 = (_e232.z * _e240);
    let _e247 = global_1.member.member_9;
    let _e255 = (1.0 / sqrt(fma(_e74.z, _e74.z, fma(_e74.x, _e74.x, (_e74.y * _e74.y)))));
    if (_e80 != true) {
        phi_1403_ = vec3<f32>(-((_e74.x * _e255)), -((_e74.y * _e255)), -((_e74.z * _e255)));
    } else {
        phi_1403_ = (_e74 * _e255);
    }
    let _e266 = phi_1403_;
    let _e267 = textureSample(global_23, global_18, _e75);
    let _e273 = fma(_e267.z, 2.007874011993408, -1.0078740119934082);
    let _e274 = (fma(_e267.x, 2.007874011993408, -1.0078740119934082) * _e247);
    let _e275 = (fma(_e267.y, 2.007874011993408, -1.0078740119934082) * _e247);
    let _e280 = (1.0 / sqrt(fma(_e273, _e273, fma(_e274, _e274, (_e275 * _e275)))));
    let _e281 = (_e274 * _e280);
    let _e282 = (_e275 * _e280);
    let _e283 = (_e273 * _e280);
    let _e284 = -(vec3<f32>(_e241, _e242, _e243));
    let _e285 = dpdx(_e284);
    let _e286 = dpdy(_e284);
    let _e287 = dpdx(_e75);
    let _e288 = dpdy(_e75);
    let _e295 = fma(_e286.y, _e266.z, -((_e266.y * _e286.z)));
    let _e300 = fma(_e286.z, _e266.x, -((_e266.z * _e286.x)));
    let _e303 = fma(_e286.x, _e266.y, -((_e266.x * _e286.y)));
    let _e317 = (_e288 * fma(_e266.y, _e285.z, -((_e285.y * _e266.z))));
    let _e320 = (_e288 * fma(_e266.z, _e285.x, -((_e285.z * _e266.x))));
    let _e323 = (_e288 * fma(_e266.x, _e285.y, -((_e285.x * _e266.y))));
    let _e326 = fma(_e295, _e287.x, _e317.x);
    let _e327 = fma(_e300, _e287.x, _e320.x);
    let _e328 = fma(_e303, _e287.x, _e323.x);
    let _e330 = fma(_e295, _e287.y, _e317.y);
    let _e331 = fma(_e300, _e287.y, _e320.y);
    let _e332 = fma(_e303, _e287.y, _e323.y);
    let _e341 = (1.0 / sqrt(max(fma(_e328, _e328, fma(_e326, _e326, (_e327 * _e327))), fma(_e332, _e332, fma(_e330, _e330, (_e331 * _e331))))));
    let _e354 = fma(_e266.x, _e283, fma((_e326 * _e341), _e281, ((_e330 * _e341) * _e282)));
    let _e355 = fma(_e266.y, _e283, fma((_e327 * _e341), _e281, ((_e331 * _e341) * _e282)));
    let _e356 = fma(_e266.z, _e283, fma((_e328 * _e341), _e281, ((_e332 * _e341) * _e282)));
    let _e361 = (1.0 / sqrt(fma(_e356, _e356, fma(_e354, _e354, (_e355 * _e355)))));
    let _e362 = (_e354 * _e361);
    let _e363 = (_e355 * _e361);
    let _e364 = (_e356 * _e361);
    if (_e177.w < 0.5) {
        discard;
    } else {
        let _e368 = global_1.member.member_10;
        if ((_e368 & 1u) == 1u) {
            let _e373 = global.member.member_12;
            if ((_e373 & 4u) == 4u) {
                phi_1161_ = pow(_e208, vec3<f32>(0.45454543828964233, 0.45454543828964233, 0.45454543828964233));
            } else {
                phi_1161_ = _e208;
            }
            let _e378 = phi_1161_;
            global_22 = vec4<f32>(_e378.x, _e378.y, _e378.z, 1.0);
        } else {
            let _e387 = fma(-(_e198.z), _e202, 1.0);
            let _e414 = sqrt(fma(_e170[3].x, _e170[3].x, fma(_e170[1].x, _e170[1].x, (_e170[2].x * _e170[2].x))));
            let _e417 = ((1.0 - _e414) / (1.0 + _e414));
            let _e418 = fma(2.0, _e414, 1.0);
            let _e435 = sqrt(fma(_e170[3].y, _e170[3].y, fma(_e170[1].y, _e170[1].y, (_e170[2].y * _e170[2].y))));
            let _e438 = ((1.0 - _e435) / (1.0 + _e435));
            let _e439 = fma(2.0, _e435, 1.0);
            let _e456 = sqrt(fma(_e170[3].z, _e170[3].z, fma(_e170[1].z, _e170[1].z, (_e170[2].z * _e170[2].z))));
            let _e459 = ((1.0 - _e456) / (1.0 + _e456));
            let _e460 = fma(2.0, _e456, 1.0);
            let _e481 = (((_e170[1].x + _e170[1].y) + _e170[1].z) * 0.3333333432674408);
            let _e482 = (((_e170[2].x + _e170[2].y) + _e170[2].z) * 0.3333333432674408);
            let _e483 = (((_e170[3].x + _e170[3].y) + _e170[3].z) * 0.3333333432674408);
            let _e487 = sqrt(fma(_e483, _e483, fma(_e481, _e481, (_e482 * _e482))));
            let _e492 = fma(-(fma(-(_e198.y), _e207, 1.0)), sqrt(_e487), 1.0);
            let _e493 = (_e492 * _e492);
            let _e494 = (_e481 / _e487);
            let _e495 = (_e482 / _e487);
            let _e496 = (_e483 / _e487);
            let _e497 = fma(_e232.x, _e240, _e494);
            let _e498 = fma(_e232.y, _e240, _e495);
            let _e499 = fma(_e232.z, _e240, _e496);
            let _e504 = (1.0 / sqrt(fma(_e499, _e499, fma(_e497, _e497, (_e498 * _e498)))));
            let _e505 = (_e497 * _e504);
            let _e506 = (_e498 * _e504);
            let _e507 = (_e499 * _e504);
            let _e518 = fma(fma(_e171.x, _e174.x, -0.04000000283122063), _e203, 0.04000000283122063);
            let _e519 = fma(fma(_e171.y, _e174.y, -0.04000000283122063), _e203, 0.04000000283122063);
            let _e520 = fma(fma(_e171.z, _e174.z, -0.04000000283122063), _e203, 0.04000000283122063);
            let _e526 = pow((1.0 - max(fma(_e243, _e507, fma(_e241, _e505, (_e242 * _e506))), 1.1920928955078125e-7)), 5.0);
            let _e535 = max(fma(_e364, _e496, fma(_e362, _e494, (_e363 * _e495))), 1.1920928955078125e-7);
            let _e539 = max(fma(_e364, _e243, fma(_e362, _e241, (_e363 * _e242))), 1.1920928955078125e-7);
            let _e543 = max(fma(_e364, _e507, fma(_e362, _e505, (_e363 * _e506))), 1.1920928955078125e-7);
            let _e544 = (_e493 * _e493);
            let _e547 = fma((_e543 * _e543), fma(_e493, _e493, -1.0), 1.0);
            let _e553 = fma(-(_e493), _e493, 1.0);
            let _e560 = fma(_e535, sqrt(fma((_e539 * _e539), _e553, _e544)), (_e539 * sqrt(fma((_e535 * _e535), _e553, _e544))));
            if (_e560 > 0.0) {
                phi_2058_ = (0.5 / _e560);
            } else {
                phi_2058_ = 0.0;
            }
            let _e564 = phi_2058_;
            let _e566 = ((vec3<f32>(_e518, _e519, _e520) + vec3<f32>(((1.0 - _e518) * _e526), ((1.0 - _e519) * _e526), ((1.0 - _e520) * _e526))) * ((_e544 / ((3.1415927410125732 * _e547) * _e547)) * _e564));
            let _e580 = fma(vec3<f32>(_e186.x, _e186.y, _e186.z), vec3<f32>(_e191, _e193, _e195), vec3<f32>(fma(((_e177.x * 0.9599999785423279) * _e387), (_e170[0].x * fma(((1.0 - _e417) * (_e418 + 1.0)), pow((0.5 * (1.0 + fma(_e170[3].x, _e364, fma(_e170[1].x, _e362, (_e170[2].x * _e363))))), _e418), _e417)), ((_e566.x * ((_e170[0].x * 9.86960506439209) * _e487)) * _e535)), fma(((_e177.y * 0.9599999785423279) * _e387), (_e170[0].y * fma(((1.0 - _e438) * (_e439 + 1.0)), pow((0.5 * (1.0 + fma(_e170[3].y, _e364, fma(_e170[1].y, _e362, (_e170[2].y * _e363))))), _e439), _e438)), ((_e566.y * ((_e170[0].y * 9.86960506439209) * _e487)) * _e535)), fma(((_e177.z * 0.9599999785423279) * _e387), (_e170[0].z * fma(((1.0 - _e459) * (_e460 + 1.0)), pow((0.5 * (1.0 + fma(_e170[3].z, _e364, fma(_e170[1].z, _e362, (_e170[2].z * _e363))))), _e460), _e459)), ((_e566.z * ((_e170[0].z * 9.86960506439209) * _e487)) * _e535))));
            let _e583 = global.member.member_12;
            if ((_e583 & 2u) == 2u) {
                phi_1217_ = min(vec3<f32>(max(((_e580.x * fma(2.509999990463257, _e580.x, 0.029999999329447746)) / fma(_e580.x, fma(2.430000066757202, _e580.x, 0.5899999737739563), 0.14000000059604645)), 0.0), max(((_e580.y * fma(2.509999990463257, _e580.y, 0.029999999329447746)) / fma(_e580.y, fma(2.430000066757202, _e580.y, 0.5899999737739563), 0.14000000059604645)), 0.0), max(((_e580.z * fma(2.509999990463257, _e580.z, 0.029999999329447746)) / fma(_e580.z, fma(2.430000066757202, _e580.z, 0.5899999737739563), 0.14000000059604645)), 0.0)), vec3<f32>(1.0, 1.0, 1.0));
            } else {
                phi_1217_ = _e580;
            }
            let _e610 = phi_1217_;
            if ((_e583 & 4u) == 4u) {
                phi_1225_ = pow(_e610, vec3<f32>(0.45454543828964233, 0.45454543828964233, 0.45454543828964233));
            } else {
                phi_1225_ = _e610;
            }
            let _e615 = phi_1225_;
            global_22 = vec4<f32>(_e615.x, _e615.y, _e615.z, 1.0);
        }
    }
    return;
}

@fragment 
fn fragment_alpha_clipped(@location(0) param: vec3<f32>, @location(1) param_1: vec3<f32>, @location(2) param_2: vec2<f32>, @location(3) param_3: vec2<f32>, @location(5) param_4: u32, @builtin(view_index) param_5: i32, @builtin(front_facing) param_6: bool, @location(4) param_7: u32) -> @location(0) vec4<f32> {
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
