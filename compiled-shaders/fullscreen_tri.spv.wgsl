struct VertexOutput {
    @location(0) member: vec2<f32>,
    @builtin(position) member_1: vec4<f32>,
}

var<private> global: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 1.0);
var<private> global_1: i32;
var<private> global_2: vec2<f32>;

fn function() {
    let _e13 = global_1;
    global_2 = vec2<f32>(f32(((_e13 << bitcast<u32>(1)) & 2)), f32((_e13 & 2)));
    let _e21 = global_2;
    global = vec4<f32>(fma(2.0, _e21.x, -1.0), fma(2.0, _e21.y, -1.0), 0.0, 1.0);
    return;
}

@vertex 
fn fullscreen_tri(@builtin(vertex_index) param: u32) -> VertexOutput {
    global_1 = i32(param);
    function();
    let _e6 = global.y;
    global.y = -(_e6);
    let _e8 = global_2;
    let _e9 = global;
    return VertexOutput(_e8, _e9);
}
