#version 150

in vec3 position;
in vec3 normal;
in mat4 model_matrix;

out vec3 v_normal;
out vec3 v_position;

uniform mat4 view;
uniform mat4 projection;

void main() {
    mat3 normal_matrix = mat3(transpose(inverse(model_matrix)));

    vec4 world_position = model_matrix * vec4(position, 1.0);
    v_position = world_position.xyz;
    v_normal = normal_matrix * normal;

    gl_Position = projection * view * world_position;
}
