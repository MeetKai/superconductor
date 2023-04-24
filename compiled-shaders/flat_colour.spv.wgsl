var<private> global: vec4<f32>;
var<private> global_1: vec3<f32>;

fn function() {
    let _e7 = global_1;
    global = vec4<f32>(_e7.x, _e7.y, _e7.z, 1.0);
    return;
}

@fragment 
fn flat_colour(@location(0) param: vec3<f32>) -> @location(0) vec4<f32> {
    global_1 = param;
    function();
    let _e3 = global;
    return _e3;
}
