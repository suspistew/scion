// shader.vert
#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in vec2 a_tex_translation;
layout(location=0) out vec2 v_tex_translation;

layout(set=1, binding=0)
uniform Uniforms {
    mat4 model_trans;
    mat4 camera_view;
};

void main() {
    v_tex_translation = a_tex_translation;
    gl_Position =  camera_view * (model_trans * vec4(a_position, 1.0));
}