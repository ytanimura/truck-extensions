#version 450

layout(location = 0) in vec3 position;
layout(location = 2) in vec3 vertex_normal;

layout(set = 0, binding = 0) uniform Camera {
    mat4 camera_matrix;
    mat4 camera_projection;
};

layout(set = 0, binding = 2) uniform Scene {
    float iTime;
    uint nlights;
};

layout(location = 0) out vec4 color;

vec4 sphericalColor(in vec3 dir);

void main() {
    vec3 camera_dir = normalize(position - camera_matrix[3].xyz);
    vec3 normal = normalize(vertex_normal);
    vec3 dir = reflect(camera_dir, normal);
    color = sphericalColor(dir);
}
