#version 140

in vec3 Normal;

out vec4 norm;

void main() {
    norm = vec4((Normal + 1.0) / 2.0, 1.0);
}
