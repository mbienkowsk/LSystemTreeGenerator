#version 140

in vec3 v_normal;
in vec3 v_position;

out vec4 color;

uniform vec3 u_light_pos;
uniform vec3 u_view_pos;
uniform int u_shading_mode; // 0 = Flat, 1 = Gouraud, 2 = Phong
uniform int u_color_mode; // 0 = Use material, 1 = Interpolate by height

uniform vec3 u_interpolation_color_low;
uniform vec3 u_interpolation_color_high;
uniform float u_total_height;

uniform vec3 u_material_ambient;
uniform vec3 u_material_diffuse;
uniform vec3 u_material_specular;

const vec3 light_color = vec3(1.0, 1.0, 1.0);
const vec3 dark_color = vec3(0.0, 0.0, 0.0);

vec3 interpolate_color() {
    float fragment_height = v_position.y;
    float height_factor = clamp(v_position.y / u_total_height, 0.0, 1.0);
    return mix(u_interpolation_color_low, u_interpolation_color_high, height_factor);
}

void flat_shading(vec3 object_color) {
    color = vec4(object_color, 1.0);
}

void gouraud_shading(vec3 object_color) {
    float brightness = dot(normalize(v_normal), normalize(u_light_pos - v_position));
    color = vec4(mix(dark_color, object_color, brightness), 1.0);
}

void phong_shading(vec3 object_color) {
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

void main() {
    vec3 object_color = interpolate_color();

    if (u_shading_mode == 0) {
        flat_shading(object_color);
    } else if (u_shading_mode == 1) {
        gouraud_shading(object_color);
    } else {
        phong_shading(object_color);
    }
}