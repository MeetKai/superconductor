# Superconductor
a work-in-progress 3d renderer built on top of `wgpu`

It primarily targets web browsers via WebGL2, with VR and AR support via WebXR. It also has some basic support for compiling to a native binary.

There are 2 main components:
- `renderer-core` which contains code for rendering pipeline creation and asset loading
- `superconductor` itself which contains a number of Bevy ECS systems for asset loading and rendering inside a rendering loop

## Features and limitations

Superconductor is limited in scope to rendering 3D GLTF models. The main goal is to render a large variety of 3D models quickly with low overhead. It contains support for:

- Loading both GLTF and GLB 3D models
- Image-based lighting via cubemaps
- Rendering both PBR and unlit GLTF models
- A limited degree of support for animated models

It has a few limitations in order to ensure performance:

- No custom shader support. Superconductor tries to use a über-shader model where a few large shaders are used to render all models, instead of many specialised shaders
- No capabilities for 2D rendering
- No support for models that use vertex-colours.
