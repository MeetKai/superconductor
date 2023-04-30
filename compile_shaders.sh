rm compiled-shaders/*

cd rust-gpu-cli-builder
cargo run --release -- ../shaders --multimodule --output ../compiled-shaders --capabilities MultiView --extensions SPV_KHR_multiview
cd ..
for file in compiled-shaders/*.spv; do
    path=$(realpath $file)
    cd ../../spirv-extra-opt-passes/spirv-extra-opt
    cd ../../work/superconductor
    spirv-val $file || echo $file;
done;

glslc granite-shaders/bc6.frag -o compiled-shaders/bc6.spv
spirv-opt compiled-shaders/bc6.spv -O -o compiled-shaders/bc6.spv

spirv-location-injector compiled-shaders/vertex.spv compiled-shaders/fragment.spv compiled-shaders/fragment.spv
spirv-location-injector compiled-shaders/vertex.spv compiled-shaders/fragment_alpha_clipped.spv compiled-shaders/fragment_alpha_clipped.spv
spirv-location-injector compiled-shaders/vertex.spv compiled-shaders/fragment_alpha_blended.spv compiled-shaders/fragment_alpha_blended.spv

spirv-location-injector compiled-shaders/single_view_vertex.spv compiled-shaders/single_view_fragment.spv compiled-shaders/single_view_fragment.spv
spirv-location-injector compiled-shaders/single_view_vertex.spv compiled-shaders/single_view_fragment_alpha_clipped.spv compiled-shaders/single_view_fragment_alpha_clipped.spv
spirv-location-injector compiled-shaders/single_view_vertex.spv compiled-shaders/single_view_fragment_alpha_blended.spv compiled-shaders/single_view_fragment_alpha_blended.spv

spirv-location-injector compiled-shaders/fullscreen_tri.spv compiled-shaders/bc6.spv compiled-shaders/bc6.spv
