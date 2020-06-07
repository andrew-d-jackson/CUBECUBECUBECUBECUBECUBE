#version 140
in vec2 Texcoords;
out vec4 color;
uniform sampler2D diffuse_textrue;
 
void main() {
    vec3 diffuse_color = texture(diffuse_textrue, Texcoords).rgb;
    color = vec4(diffuse_color, 1.0);
}
