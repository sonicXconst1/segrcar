#version 450
layout(location = 0) in vec4 i_Color;
layout(location = 0) out vec4 o_Target;

layout (set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};

void main() {
    vec4 output_color = i_Color;

// If depth testing is disabled, then manually always draw.
#ifndef LINESHADER_DEPTH_TEST
    gl_FragDepth = 0.0;
#endif

    o_Target = output_color;
}
