#version 450
layout(location = 0) in vec3 Vertex_Position;

layout(set = 0, binding = 0) uniform CameraViewProj { mat4 ViewProj; };

layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};

layout(set = 2, binding = 0) readonly buffer LineShader_points { vec3[] Points; };

const uint VERTICES_PER_LINE = 4;

void main() {
    vec3 pos = Points[gl_VertexIndex];
    gl_Position = ViewProj * vec4(pos, 1.0);
}
