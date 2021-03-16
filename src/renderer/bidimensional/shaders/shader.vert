// shader.vert
#version 450

layout(location=0) in vec3 a_position;
// Changed
layout(location=1) in vec2 a_tex_coords;

// Changed
layout(location=0) out vec2 v_tex_coords;

void main() {
    // Changed
    v_tex_coords = a_tex_coords;
    gl_Position = vec4(a_position, 1.0);
}