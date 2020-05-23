#version 140
in vec3 Position;

out vec4 color;

void main() {
    color = vec4(Position, 1.0);
}
