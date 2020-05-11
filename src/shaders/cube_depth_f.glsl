#version 140

out float depth;

void main() {
    depth = gl_FragCoord.z * 1000;
}