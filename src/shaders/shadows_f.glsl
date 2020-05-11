#version 140

uniform sampler2D sunDepth;
uniform sampler2D cameraDepth;
uniform sampler2D cameraColor;

uniform mat4 sunProjection;
uniform mat4 sunView;
uniform mat4 cameraProjection;
uniform mat4 cameraView;

in vec2 Texcoord;
out vec4 color;

void main() {
    float z = texture(cameraDepth, Texcoord).x * 2.0 - 1;
    vec4 clipSpacePosition = vec4(Texcoord * 2.0 - 1.0, z, 1.0);
    vec4 viewSpacePosition = inverse(cameraProjection) * clipSpacePosition;
    viewSpacePosition /= viewSpacePosition.w;
    vec4 inWorldSpace = inverse(cameraView) * viewSpacePosition;
    vec4 inSunSpace = sunProjection * sunView * vec4(inWorldSpace.xyz, 1.0);

    float sunDepth = texture(sunDepth, (inSunSpace.xy + 1) / 2 ).r;
    float acDistance = distance(sunDepth * 2.0 - 1, inSunSpace.z);
    if (acDistance > 0.001) {
        acDistance = 0.5;
    } else {
        acDistance = 1;
    }

    vec4 cameraColorVal = texture(cameraColor, Texcoord);

    vec3 cameraColorValShadowed = mix(cameraColorVal.rgb, cameraColorVal.rgb * acDistance, 1);

    color = vec4(cameraColorValShadowed, 1.0);
}