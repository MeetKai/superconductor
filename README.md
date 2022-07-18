# Superconductor
a work-in-progress a 3d renderer built on top of [wgpu]. It primarily targets web browsers via WebGL2, with VR and AR support via WebXR. It also has some basic support for compiling to a native binary. There are 2 main components, 'renderer-core' which contains code for rendering pipeline creation and asset loading, and 'superconductor' itself which contains a number of bevy ECS systems for asset loading and rendering inside a rendering loop.

## Features and limitations

Superconductor is limited in scope to rendering 3d gltf models. The main goal is to render a large variety of 3d models quickly with low overhead. It contains support for:

- Loading both 'gltf' and 'glb' 3d models
- Image-based lighting via cubemaps
- Rendering both PBR and unlit gltf models
- A limited degree of support for animated models.

It has a few limitations in order to ensure performance:

- No custom shader support. Superconductor tries to use a über-shader model where a few large shaders are used to render all models, instead of many specialised shaders
- No capabilities for 2d rendering
- No support for models that use vertex-colours.
