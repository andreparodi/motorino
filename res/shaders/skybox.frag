#version 330

in vec3 texture_coords;
out vec4 out_Color;

uniform samplerCube day_cube_map;
uniform samplerCube night_cube_map;
uniform vec3 day_sky_colour;
uniform vec3 night_sky_colour;
uniform float blend_factor;

const float lower_limit = 0.0;
const float upper_limit = 30.0;

void main(void){
    vec4 day_texture = texture(day_cube_map, normalize(texture_coords));
    vec4 night_texture = texture(night_cube_map, normalize(texture_coords));
    vec4 final_color = mix(day_texture, night_texture, blend_factor);

    float factor = (texture_coords.y - lower_limit) / (upper_limit - lower_limit);
    factor = clamp(factor, 0.0, 1.0);
    vec3 final_sky_colour = mix(day_sky_colour, night_sky_colour, blend_factor);

    out_Color = mix(vec4(final_sky_colour, 1.0), final_color, factor);
//    out_Color = vec4(normalize(texture_coords), 1.0);
}