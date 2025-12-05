#version 140

in vec3 v_normal;
in vec3 v_position;

out vec4 color;

uniform vec3 u_light_pos;
uniform vec3 u_view_pos;
uniform int u_shading_mode; // 0 = Flat, 1 = Gouraud, 2 = Phong
uniform vec3 u_interpolation_color_low;
uniform vec3 u_interpolation_color_high;
uniform float u_total_height;

const vec3 light_color = vec3(1.0, 1.0, 1.0);
const vec3 dark_color = vec3(0.0, 0.0, 0.0);

void main() {
    // Interpolate the object color based on height
    float height_factor = clamp(v_position.y / u_total_height, 0.0, 1.0);
    vec3 object_color = mix(u_interpolation_color_low, u_interpolation_color_high, height_factor);
    
    // flat
    if (u_shading_mode == 0) {
        color = vec4(object_color, 1.0);
        return;
    }

    // gourard
    if (u_shading_mode == 1) {
        float brightness = dot(normalize(v_normal), normalize(u_light_pos - v_position));
        color = vec4(mix(dark_color, object_color, brightness), 1.0);
        return;
    }

    // phong
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