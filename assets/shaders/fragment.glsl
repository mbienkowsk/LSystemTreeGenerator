#version 140

in vec3 v_normal;
in vec3 v_position;

out vec4 color;

uniform vec3 u_light_pos;
uniform vec3 u_view_pos;
uniform bool u_enable_phong;

const vec3 object_color = vec3(1.0, 0.0, 0.0);
const vec3 light_color = vec3(1.0, 1.0, 1.0);
const vec3 dark_color = vec3(0.0, 0.0, 0.0);

void main() {
    if (!u_enable_phong) {
        float brightness = dot(normalize(v_normal), normalize(u_light_pos));
        color = vec4(mix(dark_color, object_color, brightness), 1.0);
        return;
    }

    // ambient
    float ambient_strength = 0.1;
    vec3 ambient = ambient_strength * light_color;

    // diffuse
    vec3 norm = normalize(v_normal);
    vec3 light_dir = normalize(u_light_pos - v_position);
    float diff = max(dot(norm, light_dir), 0.0);
    vec3 diffuse = diff * light_color;

    // specular
    float specular_strength = 0.5;
    vec3 view_dir = normalize(u_view_pos - v_position);
    vec3 halfway_dir = normalize(light_dir + view_dir);
    float spec = pow(max(dot(norm, halfway_dir), 0.0), 32.0);
    vec3 specular = specular_strength * spec * light_color;

    vec3 result = (ambient + diffuse + specular) * object_color;
    color = vec4(result, 1.0);
}