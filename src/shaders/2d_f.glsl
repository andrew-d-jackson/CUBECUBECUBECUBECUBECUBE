#version 140

uniform sampler2D texFramebuffer;

in vec2 Texcoord;
out vec4 color;

void main() {
    color = vec4(texture(texFramebuffer, Texcoord).rgb, 1.0);
}