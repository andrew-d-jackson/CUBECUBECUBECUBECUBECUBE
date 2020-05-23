#version 140

in vec3 position;

uniform mat4 projection;
uniform mat4 model;
uniform mat4 camera;

void main() {
    gl_Position = projection * camera * model * vec4(position, 1.0);
}