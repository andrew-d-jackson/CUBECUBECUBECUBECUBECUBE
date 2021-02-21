#version 140

in vec3 position;
in vec2 texcoords;
in uint texindex;

out vec2 Texcoords;
flat out uint Texindex;

uniform mat4 projection;
uniform mat4 model;
uniform mat4 camera;

void main() {
    Texcoords = texcoords;
    Texindex = texindex;
    gl_Position = projection * camera * model * vec4(position, 1.0);
}