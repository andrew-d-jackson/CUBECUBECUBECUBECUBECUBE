#version 140

in vec3 position;
in vec3 normal;
in vec3 color;

out vec3 Normal;
out vec3 Color;
out vec4 SunPosition;
out vec4 SunPosition2;

uniform mat4 projection;
uniform mat4 model;
uniform mat4 camera;
uniform mat4 sunProjection;
uniform mat4 sunView;
uniform mat4 sunProjection2;


void main() {
    Normal = normal;
    Color = color;
    SunPosition = (sunProjection * sunView * model * vec4(position, 1.0));
    SunPosition2 = (sunProjection2 * sunView * model * vec4(position, 1.0));
    gl_Position = projection * camera * model * vec4(position, 1.0);
}