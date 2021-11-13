#version 450
layout(location = 0) in vec3 Vertex_Position;
layout(location = 0) out vec4 o_Color;

layout(set = 0, binding = 0) uniform CameraViewProj { mat4 ViewProj; };

layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};

layout(set = 2, binding = 0) readonly buffer LineShader_colors { vec4[] Colors; };


void main() {
    vec4 pos = vec4(Vertex_Position, 1.0);
    o_Color = Colors[gl_VertexIndex];
    gl_Position = ViewProj * Model * pos;
}
