#version 140
in vec3 Normal;
in vec3 Color;

out vec4 color;
void main() {
    float brightness = dot(normalize(Normal), normalize(vec3(50.0, -70.0, 30.0)));
    vec3 reg_color = vec3(Color.x / 255, Color.y / 255, Color.z / 255);
    vec3 dark_color = reg_color / 2;
    color = vec4(mix(dark_color, reg_color, brightness), 1.0);
}