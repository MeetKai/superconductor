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
    var phi_1056_: array<vec3<f32>,4u>;
    var phi_1110_: vec3<f32>;
    var phi_1133_: vec3<f32>;
    var phi_1414_: vec3<f32>;
    var phi_2048_: f32;
    var phi_1210_: vec3<f32>;
    var phi_1218_: vec3<f32>;

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
        phi_1056_ = array<vec3<f32>,4u>(vec3<f32>(_e109.x, _e109.y, _e109.z), vec3<f32>(fma(_e114.x, 2.007874011993408, -1.0078740119934082), fma(_e114.y, 2.007874011993408, -1.0078740119934082), fma(_e114.z, 2.007874011993408, -1.0078740119934082)), vec3<f32>(fma(_e122.x, 2.007874011993408, -1.0078740119934082), fma(_e122.y, 2.007874011993408, -1.0078740119934082), fma(_e122.z, 2.007874011993408, -1.0078740119934082)), vec3<f32>(fma(_e130.x, 2.007874011993408, -1.0078740119934082), fma(_e130.y, 2.007874011993408, -1.0078740119934082), fma(_e130.z, 2.007874011993408, -1.0078740119934082)));
    } else {
        let _e139 = textureSampleLevel(global_14, global_9, _e76, 0.0);
        let _e144 = textureSampleLevel(global_15, global_9, _e76, 0.0);
        let _e152 = textureSampleLevel(global_16, global_9, _e76, 0.0);
        let _e160 = textureSampleLevel(global_17, global_9, _e76, 0.0);
        phi_1056_ = array<vec3<f32>,4u>(vec3<f32>(_e139.x, _e139.y, _e139.z), vec3<f32>(fma(_e144.x, 2.007874011993408, -1.0078740119934082), fma(_e144.y, 2.007874011993408, -1.0078740119934082), fma(_e144.z, 2.007874011993408, -1.0078740119934082)), vec3<f32>(fma(_e152.x, 2.007874011993408, -1.0078740119934082), fma(_e152.y, 2.007874011993408, -1.0078740119934082), fma(_e152.z, 2.007874011993408, -1.0078740119934082)), vec3<f32>(fma(_e160.x, 2.007874011993408, -1.0078740119934082), fma(_e160.y, 2.007874011993408, -1.0078740119934082), fma(_e160.z, 2.007874011993408, -1.0078740119934082)));
    }
    let _e170 = phi_1056_;
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
    let _e211 = global_1.member.member_10;
    if ((_e211 & 1u) == 1u) {
        let _e216 = global.member.member_12;
        if ((_e216 & 4u) == 4u) {
            phi_1110_ = pow(_e208, vec3<f32>(0.45454543828964233, 0.45454543828964233, 0.45454543828964233));
        } else {
            phi_1110_ = _e208;
        }
        let _e221 = phi_1110_;
        global_22 = vec4<f32>(_e221.x, _e221.y, _e221.z, _e177.w);
    } else {
        if (_e79 == 0) {
            let _e229 = global.member.member_6;
            let _e232 = global.member.member_7;
            let _e235 = global.member.member_8;
            phi_1133_ = vec3<f32>(_e229, _e232, _e235);
        } else {
            let _e239 = global.member.member_9;
            let _e242 = global.member.member_10;
            let _e245 = global.member.member_11;
            phi_1133_ = vec3<f32>(_e239, _e242, _e245);
        }
        let _e248 = phi_1133_;
        let _e249 = (_e248 - _e73);
        let _e257 = (1.0 / sqrt(fma(_e249.z, _e249.z, fma(_e249.x, _e249.x, (_e249.y * _e249.y)))));
        let _e258 = (_e249.x * _e257);
        let _e259 = (_e249.y * _e257);
        let _e260 = (_e249.z * _e257);
        let _e264 = global_1.member.member_9;
        let _e272 = (1.0 / sqrt(fma(_e74.z, _e74.z, fma(_e74.x, _e74.x, (_e74.y * _e74.y)))));
        if (_e80 != true) {
            phi_1414_ = vec3<f32>(-((_e74.x * _e272)), -((_e74.y * _e272)), -((_e74.z * _e272)));
        } else {
            phi_1414_ = (_e74 * _e272);
        }
        let _e283 = phi_1414_;
        let _e284 = textureSample(global_23, global_18, _e75);
        let _e290 = fma(_e284.z, 2.007874011993408, -1.0078740119934082);
        let _e291 = (fma(_e284.x, 2.007874011993408, -1.0078740119934082) * _e264);
        let _e292 = (fma(_e284.y, 2.007874011993408, -1.0078740119934082) * _e264);
        let _e297 = (1.0 / sqrt(fma(_e290, _e290, fma(_e291, _e291, (_e292 * _e292)))));
        let _e298 = (_e291 * _e297);
        let _e299 = (_e292 * _e297);
        let _e300 = (_e290 * _e297);
        let _e301 = -(vec3<f32>(_e258, _e259, _e260));
        let _e302 = dpdx(_e301);
        let _e303 = dpdy(_e301);
        let _e304 = dpdx(_e75);
        let _e305 = dpdy(_e75);
        let _e312 = fma(_e303.y, _e283.z, -((_e283.y * _e303.z)));
        let _e317 = fma(_e303.z, _e283.x, -((_e283.z * _e303.x)));
        let _e320 = fma(_e303.x, _e283.y, -((_e283.x * _e303.y)));
        let _e334 = (_e305 * fma(_e283.y, _e302.z, -((_e302.y * _e283.z))));
        let _e337 = (_e305 * fma(_e283.z, _e302.x, -((_e302.z * _e283.x))));
        let _e340 = (_e305 * fma(_e283.x, _e302.y, -((_e302.x * _e283.y))));
        let _e343 = fma(_e312, _e304.x, _e334.x);
        let _e344 = fma(_e317, _e304.x, _e337.x);
        let _e345 = fma(_e320, _e304.x, _e340.x);
        let _e347 = fma(_e312, _e304.y, _e334.y);
        let _e348 = fma(_e317, _e304.y, _e337.y);
        let _e349 = fma(_e320, _e304.y, _e340.y);
        let _e358 = (1.0 / sqrt(max(fma(_e345, _e345, fma(_e343, _e343, (_e344 * _e344))), fma(_e349, _e349, fma(_e347, _e347, (_e348 * _e348))))));
        let _e371 = fma(_e283.x, _e300, fma((_e343 * _e358), _e298, ((_e347 * _e358) * _e299)));
        let _e372 = fma(_e283.y, _e300, fma((_e344 * _e358), _e298, ((_e348 * _e358) * _e299)));
        let _e373 = fma(_e283.z, _e300, fma((_e345 * _e358), _e298, ((_e349 * _e358) * _e299)));
        let _e378 = (1.0 / sqrt(fma(_e373, _e373, fma(_e371, _e371, (_e372 * _e372)))));
        let _e379 = (_e371 * _e378);
        let _e380 = (_e372 * _e378);
        let _e381 = (_e373 * _e378);
        let _e386 = fma(-(_e198.z), _e202, 1.0);
        let _e413 = sqrt(fma(_e170[3].x, _e170[3].x, fma(_e170[1].x, _e170[1].x, (_e170[2].x * _e170[2].x))));
        let _e416 = ((1.0 - _e413) / (1.0 + _e413));
        let _e417 = fma(2.0, _e413, 1.0);
        let _e434 = sqrt(fma(_e170[3].y, _e170[3].y, fma(_e170[1].y, _e170[1].y, (_e170[2].y * _e170[2].y))));
        let _e437 = ((1.0 - _e434) / (1.0 + _e434));
        let _e438 = fma(2.0, _e434, 1.0);
        let _e455 = sqrt(fma(_e170[3].z, _e170[3].z, fma(_e170[1].z, _e170[1].z, (_e170[2].z * _e170[2].z))));
        let _e458 = ((1.0 - _e455) / (1.0 + _e455));
        let _e459 = fma(2.0, _e455, 1.0);
        let _e480 = (((_e170[1].x + _e170[1].y) + _e170[1].z) * 0.3333333432674408);
        let _e481 = (((_e170[2].x + _e170[2].y) + _e170[2].z) * 0.3333333432674408);
        let _e482 = (((_e170[3].x + _e170[3].y) + _e170[3].z) * 0.3333333432674408);
        let _e486 = sqrt(fma(_e482, _e482, fma(_e480, _e480, (_e481 * _e481))));
        let _e491 = fma(-(fma(-(_e198.y), _e207, 1.0)), sqrt(_e486), 1.0);
        let _e492 = (_e491 * _e491);
        let _e493 = (_e480 / _e486);
        let _e494 = (_e481 / _e486);
        let _e495 = (_e482 / _e486);
        let _e496 = fma(_e249.x, _e257, _e493);
        let _e497 = fma(_e249.y, _e257, _e494);
        let _e498 = fma(_e249.z, _e257, _e495);
        let _e503 = (1.0 / sqrt(fma(_e498, _e498, fma(_e496, _e496, (_e497 * _e497)))));
        let _e504 = (_e496 * _e503);
        let _e505 = (_e497 * _e503);
        let _e506 = (_e498 * _e503);
        let _e517 = fma(fma(_e171.x, _e174.x, -0.04000000283122063), _e203, 0.04000000283122063);
        let _e518 = fma(fma(_e171.y, _e174.y, -0.04000000283122063), _e203, 0.04000000283122063);
        let _e519 = fma(fma(_e171.z, _e174.z, -0.04000000283122063), _e203, 0.04000000283122063);
        let _e525 = pow((1.0 - max(fma(_e260, _e506, fma(_e258, _e504, (_e259 * _e505))), 1.1920928955078125e-7)), 5.0);
        let _e534 = max(fma(_e381, _e495, fma(_e379, _e493, (_e380 * _e494))), 1.1920928955078125e-7);
        let _e538 = max(fma(_e381, _e260, fma(_e379, _e258, (_e380 * _e259))), 1.1920928955078125e-7);
        let _e542 = max(fma(_e381, _e506, fma(_e379, _e504, (_e380 * _e505))), 1.1920928955078125e-7);
        let _e543 = (_e492 * _e492);
        let _e546 = fma((_e542 * _e542), fma(_e492, _e492, -1.0), 1.0);
        let _e552 = fma(-(_e492), _e492, 1.0);
        let _e559 = fma(_e534, sqrt(fma((_e538 * _e538), _e552, _e543)), (_e538 * sqrt(fma((_e534 * _e534), _e552, _e543))));
        if (_e559 > 0.0) {
            phi_2048_ = (0.5 / _e559);
        } else {
            phi_2048_ = 0.0;
        }
        let _e563 = phi_2048_;
        let _e565 = ((vec3<f32>(_e517, _e518, _e519) + vec3<f32>(((1.0 - _e517) * _e525), ((1.0 - _e518) * _e525), ((1.0 - _e519) * _e525))) * ((_e543 / ((3.1415927410125732 * _e546) * _e546)) * _e563));
        let _e579 = fma(vec3<f32>(_e186.x, _e186.y, _e186.z), vec3<f32>(_e191, _e193, _e195), vec3<f32>(fma(((_e177.x * 0.9599999785423279) * _e386), (_e170[0].x * fma(((1.0 - _e416) * (_e417 + 1.0)), pow((0.5 * (1.0 + fma(_e170[3].x, _e381, fma(_e170[1].x, _e379, (_e170[2].x * _e380))))), _e417), _e416)), ((_e565.x * ((_e170[0].x * 9.86960506439209) * _e486)) * _e534)), fma(((_e177.y * 0.9599999785423279) * _e386), (_e170[0].y * fma(((1.0 - _e437) * (_e438 + 1.0)), pow((0.5 * (1.0 + fma(_e170[3].y, _e381, fma(_e170[1].y, _e379, (_e170[2].y * _e380))))), _e438), _e437)), ((_e565.y * ((_e170[0].y * 9.86960506439209) * _e486)) * _e534)), fma(((_e177.z * 0.9599999785423279) * _e386), (_e170[0].z * fma(((1.0 - _e458) * (_e459 + 1.0)), pow((0.5 * (1.0 + fma(_e170[3].z, _e381, fma(_e170[1].z, _e379, (_e170[2].z * _e380))))), _e459), _e458)), ((_e565.z * ((_e170[0].z * 9.86960506439209) * _e486)) * _e534))));
        let _e582 = global.member.member_12;
        if ((_e582 & 2u) == 2u) {
            phi_1210_ = min(vec3<f32>(max(((_e579.x * fma(2.509999990463257, _e579.x, 0.029999999329447746)) / fma(_e579.x, fma(2.430000066757202, _e579.x, 0.5899999737739563), 0.14000000059604645)), 0.0), max(((_e579.y * fma(2.509999990463257, _e579.y, 0.029999999329447746)) / fma(_e579.y, fma(2.430000066757202, _e579.y, 0.5899999737739563), 0.14000000059604645)), 0.0), max(((_e579.z * fma(2.509999990463257, _e579.z, 0.029999999329447746)) / fma(_e579.z, fma(2.430000066757202, _e579.z, 0.5899999737739563), 0.14000000059604645)), 0.0)), vec3<f32>(1.0, 1.0, 1.0));
        } else {
            phi_1210_ = _e579;
        }
        let _e609 = phi_1210_;
        if ((_e582 & 4u) == 4u) {
            phi_1218_ = pow(_e609, vec3<f32>(0.45454543828964233, 0.45454543828964233, 0.45454543828964233));
        } else {
            phi_1218_ = _e609;
        }
        let _e614 = phi_1218_;
        global_22 = vec4<f32>(_e614.x, _e614.y, _e614.z, _e177.w);
    }
    return;
}

@fragment 
fn fragment_alpha_blended(@location(0) param: vec3<f32>, @location(1) param_1: vec3<f32>, @location(2) param_2: vec2<f32>, @location(3) param_3: vec2<f32>, @location(5) param_4: u32, @builtin(view_index) param_5: i32, @builtin(front_facing) param_6: bool, @location(4) param_7: u32) -> @location(0) vec4<f32> {
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
