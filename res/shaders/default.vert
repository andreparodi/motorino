#version 330 core
layout (location = 0) in vec3 position;
layout (location = 1) in vec2 texture_coords;
layout (location = 2) in vec3 normal;

out vec2 passed_texture_coords;
out vec3 surface_normal;
out vec3 to_light_vector;
out vec3 to_camera_vector;
out vec3 pos;
out float visibility;

uniform mat4 transformation_matrix;
uniform mat4 projection_matrix;
uniform mat4 view_matrix;
uniform vec3 light_position;
uniform float fog_density;
uniform float fog_gradient;


void main() {
    vec4 world_position = transformation_matrix * vec4(position, 1.0);
    vec4 position_relative_to_camera = view_matrix * world_position;
    gl_Position = projection_matrix * position_relative_to_camera;
    passed_texture_coords = texture_coords;

    surface_normal = (transformation_matrix * vec4(normal, 0.0)).xyz;
    pos = surface_normal;
    to_light_vector = light_position - world_position.xyz;
    to_camera_vector = (inverse(view_matrix) * vec4(0.0, 0.0, 0.0, 1.0)).xyz -  world_position.xyz;

    float distance = length(position_relative_to_camera.xyz);
    visibility = exp(-pow((distance * fog_density), fog_gradient));
    visibility = clamp(visibility, 0.0, 1.0);
}