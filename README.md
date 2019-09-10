# motorino
opengl rendering playground with rust

First rust project to learn rust and opengl

currently implemented:
* loading 3d models
* rendering ground
* heightmaps for ground
* movement of character
* integrated imgui for debug info
* camera to follow player
* skybox
* blending of texture to create ground with path
* uses specs ECS

todo
* implement frustum culling to limit the number of objects rendered to those in the view
* try rayon parallel iterator par_join
* error handling - understand the best approach in rus
* point lights
* collision detection
* better main loop that doesn't max out cpu
* water effect
