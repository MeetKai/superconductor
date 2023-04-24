var<private> global: vec4<f32>;
var<private> global_1: vec2<f32>;
@group(1) @binding(0) 
var global_2: sampler;
@group(1) @binding(1) 
var global_3: texture_2d<f32>;

fn function() {
    let _e18 = global_1;
    let _e19 = textureSample(global_3, global_2, _e18);
    let _e43 = pow(min(vec3<f32>(max(((_e19.x * fma(2.509999990463257, _e19.x, 0.029999999329447746)) / fma(_e19.x, fma(2.430000066757202, _e19.x, 0.5899999737739563), 0.14000000059604645)), 0.0), max(((_e19.y * fma(2.509999990463257, _e19.y, 0.029999999329447746)) / fma(_e19.y, fma(2.430000066757202, _e19.y, 0.5899999737739563), 0.14000000059604645)), 0.0), max(((_e19.z * fma(2.509999990463257, _e19.z, 0.029999999329447746)) / fma(_e19.z, fma(2.430000066757202, _e19.z, 0.5899999737739563), 0.14000000059604645)), 0.0)), vec3<f32>(1.0, 1.0, 1.0)), vec3<f32>(0.45454543828964233, 0.45454543828964233, 0.45454543828964233));
    global = vec4<f32>(_e43.x, _e43.y, _e43.z, 1.0);
    return;
}

@fragment 
fn single_view__tonemap(@location(0) param: vec2<f32>) -> @location(0) vec4<f32> {
    global_1 = param;
    function();
    let _e3 = global;
    return _e3;
}
