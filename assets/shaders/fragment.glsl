#version 140

in vec3 v_normal;
in vec3 v_position;

out vec4 color;

uniform int u_shading_mode; // 0 = Flat, 1 = Gouraud, 2 = Phong
uniform int u_color_mode; // 0 = Use material, 1 = Interpolate by height

uniform vec3 u_light_pos;
uniform vec3 u_view_pos;

uniform vec3 u_interpolation_color_low;
uniform vec3 u_interpolation_color_high;
uniform float u_total_height;

uniform vec3 u_material_ambient;
uniform vec3 u_material_diffuse;
uniform vec3 u_material_specular;
uniform float u_material_shininess;

const vec3 light_color = vec3(1.0, 1.0, 1.0);
const vec3 dark_color = vec3(0.0, 0.0, 0.0);

struct Material {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    float shininess;
};


vec3 interpolate_color() {
    float fragment_height = v_position.y;
    float height_factor = clamp(v_position.y / u_total_height, 0.0, 1.0);
    return mix(u_interpolation_color_low, u_interpolation_color_high, height_factor);
}

Material get_material() {
    if (u_color_mode == 0) {
        return Material(
        u_material_ambient,
        u_material_diffuse,
        u_material_specular,
        u_material_shininess
        );
    } else {
        vec3 object_color = interpolate_color();
        return Material(
        object_color * 0.1,
        object_color,
        vec3(0.3, 0.3, 0.3),
        10.0
        );
    }
}

void flat_shading(vec3 object_color) {
    color = vec4(object_color, 1.0);
}

void gouraud_shading(vec3 object_color) {
    float brightness = dot(normalize(v_normal), normalize(u_light_pos - v_position));
    color = vec4(mix(dark_color, object_color, brightness), 1.0);
}


void phong_shading(Material object_material) {
    // ambient
    vec3 ambient = object_material.ambient * light_color;

    // diffuse
    vec3 norm = normalize(v_normal);
    vec3 light_dir = normalize(u_light_pos - v_position);
    float diff = max(dot(norm, light_dir), 0.0);
    vec3 diffuse = object_material.diffuse * diff * light_color;

    // specular
    vec3 view_dir = normalize(u_view_pos - v_position);
    vec3 halfway_dir = normalize(light_dir + view_dir);
    float spec = pow(max(dot(norm, halfway_dir), 0.0), object_material.shininess);
    vec3 specular = object_material.specular * spec * light_color;

    // combine
    vec3 result = ambient + diffuse + specular;
    color = vec4(result, 1.0);
}


void main() {
    Material object_material = get_material();

    if (u_shading_mode == 0) {
        flat_shading(object_material.diffuse);
    } else if (u_shading_mode == 1) {
        gouraud_shading(object_material.diffuse);
    } else {
        phong_shading(object_material);
    }
}

