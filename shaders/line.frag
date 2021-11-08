#version 450
layout(location = 0) out vec4 o_Target;

layout (set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};

layout(set = 2, binding = 1) uniform LineShader_color { vec4 Color; };

void main() {
    vec4 output_color = vec4(1.0, 0.0, 0.0, 1.0);

// If depth testing is disabled, then manually always draw.
#ifndef LINESHADER_DEPTH_TEST
    gl_FragDepth = 0.0;
#endif

    o_Target = Color;
}
