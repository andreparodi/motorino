#version 330 core
in vec3 colour;
in vec2 passed_texture_coords;
in vec3 pos;
in vec3 surface_normal;
in vec3 to_light_vector;
in vec3 to_camera_vector;
in float visibility;

out vec4 out_colour;

uniform sampler2D background_sampler;
uniform sampler2D r_sampler;
uniform sampler2D g_sampler;
uniform sampler2D b_sampler;
uniform sampler2D blend_map_sampler;
uniform vec3 light_colour;
uniform float reflectivity;
uniform float shine_damper;
uniform vec3 sky_colour;

void main() {

    vec4 blend_map_colour = texture(blend_map_sampler, passed_texture_coords);
    float background_texture_ratio = 1.0 - (blend_map_colour.r + blend_map_colour.g + blend_map_colour.b);
    vec2 tiled_coords = passed_texture_coords * 40;
    vec4 background_texture_colour = texture(background_sampler, tiled_coords) * background_texture_ratio;
    vec4 r_texture_colour = texture(r_sampler, tiled_coords) * blend_map_colour.r;
    vec4 g_texture_colour = texture(g_sampler, tiled_coords) * blend_map_colour.g;
    vec4 b_texture_colour = texture(b_sampler, tiled_coords) * blend_map_colour.b;
    vec4 final_colour = background_texture_colour + r_texture_colour + g_texture_colour + b_texture_colour;
   // vec4 final_colour = vec4(colour, 1.0);


    vec3 unit_normal = normalize(surface_normal);
    vec3 unit_light_vector = normalize(to_light_vector);
    vec3 unit_camera_vector = normalize(to_camera_vector);

    vec3 light_direction = -unit_light_vector;
    vec3 reflected_light_direction = reflect(light_direction, unit_normal);
    float specular_factor = dot(reflected_light_direction, unit_camera_vector);
    specular_factor = max(specular_factor, 0.0);
    float damp_factor = pow(specular_factor, shine_damper);
    vec3 specular_light = damp_factor * reflectivity * light_colour;

    float nDot1 = dot(unit_normal, unit_light_vector);
    float brightness = max(nDot1, 0.0);
    vec3 diffuse = brightness * light_colour;

    //vec3 pos_norm = normalize(debug_var);

    //out_colour = vec4(pos_norm, 1.0);

    out_colour = vec4(specular_light, 1.0) + vec4(diffuse, 1.0) * final_colour;
    out_colour = mix(vec4(sky_colour, 1.0), out_colour, visibility);

    //out_colour = vec4(pos, 1.0);
    //out_colour = final_colour;
    //out_colour = vec4(diffuse, 1.0) * texture(texture_sampler, passed_texture_coords);

}