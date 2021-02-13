#version 450

layout(location = 0) in vec3 vertex_direction;
layout(location = 0) out vec4 color;

struct Light {
    vec4 position;
    vec4 color;
    uvec4 light_type;
};

layout(set = 0, binding = 1) buffer Lights {
    Light iLights[];
};

layout(set = 0, binding = 2) uniform Scene {
    float iTime;
    uint nLights;
};

vec4 sphericalColor(in vec3 dir);

void main() {
    color = sphericalColor(normalize(vertex_direction));
}
