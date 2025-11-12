pub const VERTEX_SHADER_SRC: &str = "
    #version 150
    in vec3 position;
    in vec3 normal;

    out vec3 v_normal;

    uniform mat4 model;
    uniform mat4 view;
    uniform mat4 projection;
    uniform mat3 normal_matrix; // (model)^-1 * normal

    void main() {

        v_normal = normal_matrix * normal;
        gl_Position = projection * view * model * vec4(position, 1.0);
    }
";

/// gouraurd for now
pub const FRAGMENT_SHADER_SRC: &str = "
    #version 140
    in vec3 v_normal;
    out vec4 color;

    void main() {
        vec3 u_light = vec3(-1.0, 0.4, 0.9);

        float brightness = dot(normalize(v_normal), normalize(u_light));
        vec3 dark_color = vec3(0.6, 0.0, 0.0);
        vec3 regular_color = vec3(1.0, 0.0, 0.0);
        color = vec4(mix(dark_color, regular_color, brightness), 1.0);
    }
";
