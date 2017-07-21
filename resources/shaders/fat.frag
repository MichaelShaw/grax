#version 150 core

layout (std140)
uniform Locals {
	mat4 u_transform;
	vec4 u_color;
	float u_alpha_minimum;
};

uniform sampler2DArray u_texture;

in vec4 v_color;
in vec3 v_tex_coord;
in vec3 v_normal;

out vec4 f_color;

void main() {
    vec4 albedo_colour = texture(u_texture, v_tex_coord) * 0.8 + v_color * 0.2;

    vec4 final_colour = albedo_colour; // * light;
    final_colour.a = v_color.a; // ignore light's alpha

    //if(final_colour.a < 0.001) { // u_alpha_minimum
     //  discard;
    //}
    f_color = final_colour;
}


