#version 140
in vec2 Texcoords;
flat in uint Texindex;
out vec4 color;
uniform sampler2DArray diffuse_textrues;
 
void main() {
    vec3 diffuse_color = texture(diffuse_textrues, vec3(Texcoords, Texindex)).rgb;
    color = vec4(diffuse_color, 1.0);
}
