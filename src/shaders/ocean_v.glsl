#version 140

in vec3 position;

uniform mat4 projection;
uniform mat4 model;
uniform mat4 camera;

out vec4 SunPosition;
out vec4 SunPosition2;
uniform mat4 sunProjection;
uniform mat4 sunView;
uniform mat4 sunProjection2;

void main() {
    SunPosition = (sunProjection * sunView * model * vec4(position, 1.0));
    SunPosition2 = (sunProjection2 * sunView * model * vec4(position, 1.0));

    gl_Position = projection * camera * model * vec4(position, 1.0);
}