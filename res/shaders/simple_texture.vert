#version 330 core
layout (location = 0) in vec3 aPos;

out vec3 colour;
void main() {
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0f);
    colour = vec3(aPos.x + 0.5f, 1.0f, aPos.y + 0.5f);
}




#version 330 core
//layout (location = 0) in vec3 in_position;
//layout (location = 1) in vec2 texture_coords;

//out vec2 passed_texture_coords;
//out vec3 colour;

//void main() {
//    gl_Position = vec4(in_position.x, in_position.y, aPosin_position.z, 1.0f);
//    colour = vec3(aPos.x + 0.5f, 1.0f, aPos.y + 0.5f);
    //passed_texture_coords = texture_coords;
//}