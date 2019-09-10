#version 330
layout (location = 0) in vec3 position;

out vec3 texture_coords;

uniform mat4 projection_matrix;
uniform mat4 view_matrix;

void main(void){
	gl_Position = projection_matrix * view_matrix * vec4(position, 1.0);
	texture_coords = position;
}