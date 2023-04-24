var<private> global: vec4<f32>;
var<private> global_1: vec2<f32>;
@group(0) @binding(0) 
var global_2: sampler;
@group(0) @binding(1) 
var global_3: texture_2d<f32>;

fn function() {
    let _e10 = global_1;
    _ = _e10.y;
    let _e16 = textureSampleLevel(global_3, global_2, vec2<f32>(_e10.x, (1.0 - _e10.y)), 0.0);
    global = _e16;
    return;
}

@fragment 
fn blit(@location(0) param: vec2<f32>) -> @location(0) vec4<f32> {
    global_1 = param;
    function();
    let _e3 = global;
    return _e3;
}
