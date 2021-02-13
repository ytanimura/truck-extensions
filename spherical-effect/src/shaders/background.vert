#version 450

layout(location = 0) in uint idx;
layout(location = 0) out vec3 dir;

layout(set = 0, binding = 0) uniform Camera {
    mat4 camera_matrix;
    mat4 camera_projection;
};

const vec2 coords[4] = vec2[](
    vec2(-1, -1), vec2(1, -1), vec2(-1, 1), vec2(1, 1)
);

void main() {
    gl_Position = vec4(coords[idx], 1.0, 1);
    mat4 proj = camera_projection * camera_matrix;
    float f = proj[1][1];
    float asp = f / proj[0][0];
    vec4 dir4 = -f * camera_matrix[2]
        + coords[idx][0] * asp * camera_matrix[0]
        + coords[idx][1] * camera_matrix[1];
    dir = dir4.xyz;
}
