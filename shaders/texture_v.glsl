#version 140

in vec3 position;
in vec2 texcoords;

out vec2 Texcoords;

uniform mat4 projection;
uniform mat4 model;
uniform mat4 camera;

void main() {
    Texcoords = texcoords;
    gl_Position = projection * camera * model * vec4(position, 1.0);
}