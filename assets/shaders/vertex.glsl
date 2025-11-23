#version 150

in vec3 position;
in vec3 normal;
in vec4 model_matrix_0;
in vec4 model_matrix_1;
in vec4 model_matrix_2;
in vec4 model_matrix_3;

out vec3 v_normal;
out vec3 v_position;

uniform mat4 view;
uniform mat4 projection;

void main() {
    mat4 model = mat4(model_matrix_0, model_matrix_1, model_matrix_2, model_matrix_3);
    mat3 normal_matrix = mat3(transpose(inverse(model)));

    vec4 world_position = model * vec4(position, 1.0);
    v_position = world_position.xyz;
    v_normal = normal_matrix * normal;

    gl_Position = projection * view * world_position;
}
