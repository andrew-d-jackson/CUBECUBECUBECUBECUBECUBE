#version 140

uniform sampler2D lightDepth;
uniform sampler2D currentColor;
uniform sampler2D cameraDepth;
uniform sampler2D cameraColor;
uniform sampler2D cameraNormals;

uniform vec3 lightPosition;
uniform vec3 lightColor;
uniform float lightStrength;

uniform mat4 lightProjection;
uniform mat4 lightView;
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

    vec3 directionToSun = normalize(lightPosition - inWorldSpace.xyz);
    vec3 normal = normalize(texture(cameraNormals, Texcoord).xyz * 2 - 1);
    float difference = dot(directionToSun, normal);

    vec4 inSunSpace = lightProjection * lightView * vec4(inWorldSpace.xyz, 1.0);

    float sunDepth = texture(lightDepth, (inSunSpace.xy + 1) / 2 ).r;
    float acDistance = distance(sunDepth * 2.0 - 1, inSunSpace.z);
    if (sunDepth * 2.0 - 1 < inSunSpace.z - 0.0001 || difference < 0) {
        acDistance = 0;
    } else {
        acDistance = lightStrength;
    }

    if (inSunSpace.x > 1 || inSunSpace.x < -1 || inSunSpace.y > 1 || inSunSpace.y < -1) {
        acDistance = 0;
    }

    vec4 cameraColorVal = texture(currentColor, Texcoord);

    vec3 lightColor = lightColor * acDistance;
    vec3 outColor = lightColor + cameraColorVal.rgb;
    vec3 cameraColorValShadowed = mix(cameraColorVal.rgb, cameraColorVal.rgb * acDistance, 1);

   //color = vec4(cameraColorValShadowed, 1.0);
   //color = vec4(texture(cameraColor, Texcoord).xyz, 1.0);
   color = vec4(outColor, 0.0);
}