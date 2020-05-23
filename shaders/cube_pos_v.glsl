#version 140

in vec3 position;
out vec3 Position;

uniform mat4 projection;
uniform mat4 model;
uniform mat4 camera;

void main() {
    Position = (model * vec4(position, 1.0)).xyz;
    gl_Position = projection * camera * model * vec4(position, 1.0);
}