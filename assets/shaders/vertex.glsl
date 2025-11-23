#version 150

in vec3 position;
in vec3 normal;

out vec3 v_normal;
out vec3 v_position;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform mat3 normal_matrix; // (model)^-1 * normal

void main() {
    vec4 world_position = model * vec4(position, 1.0);
    v_position = world_position.xyz;
    
    v_normal = normal_matrix * normal;
    gl_Position = projection * view * world_position;
}