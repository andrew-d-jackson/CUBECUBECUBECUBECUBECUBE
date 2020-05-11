#version 140

in vec3 position;
in vec3 normal;
in vec3 color;

out vec3 Normal;
out vec3 Color;

uniform mat4 projection;
uniform mat4 model;
uniform mat4 camera;

void main() {
    Normal = normal;
    Color = color;
    gl_Position = projection * camera * model * vec4(position, 1.0);
}