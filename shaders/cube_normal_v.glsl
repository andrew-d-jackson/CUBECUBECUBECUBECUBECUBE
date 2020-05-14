#version 140

in vec3 position;
in vec3 normal;

out vec3 Normal;

uniform mat4 projection;
uniform mat4 model;
uniform mat4 camera;

void main() {
    Normal = normal;
    gl_Position = projection * camera * model * vec4(position, 1.0);
}