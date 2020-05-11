#version 140

in vec3 Normal;

out vec4 output;

void main() {
    output = vec4((Normal + 1.0) / 2.0, 1.0);
}
