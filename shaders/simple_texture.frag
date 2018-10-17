#version 330 core
in vec3 colour;
out vec4 FragColor;
void main() {
    FragColor = vec4(colour, 1.0f);
}



//#version 330 core
//in vec3 passed_texture_coords
//in vec3 colour;

//out vec4 out_colour;

//uniform smapler2D texture_sampler;

//void main() {
    //out_colour = texture(texture_sampler, passed_texture_coords);
  //  out_colour = vec4(colour, 1.0f);
//}