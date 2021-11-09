#version 450
layout(location = 0) in vec3 Vertex_Position;

layout(set = 0, binding = 0) uniform CameraViewProj { mat4 ViewProj; };

layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};


layout(set = 2, binding = 0) readonly buffer LineShader_points { vec4[] Points; };

void main() {
    vec4 pos = Points[gl_VertexIndex];
    gl_Position = ViewProj * Model * pos;
}
