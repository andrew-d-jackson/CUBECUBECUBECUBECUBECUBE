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

float shadowing(sampler2D shadowMap, vec2 projCoords, float currentDepth) {
    float shadow = 0.0;
    float bias = 0.0008;
    int samples = 1;
    int samplesTaken = 0;
    vec2 texelSize = 1.0 / textureSize(shadowMap, 0);
    for(int x = 0 - samples; x <= samples; ++x)
    {
        for(int y = 0 - samples; y <= samples; ++y)
        {
            samplesTaken += 1;
            float pcfDepth = texture(shadowMap, projCoords.xy + vec2(x, y) * texelSize).r; 
            shadow += currentDepth - bias > pcfDepth ? 0.0 : 1.0;        
        }    
    }
    shadow /= float(samplesTaken);
    return shadow;
}

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

    float acDistance = shadowing(lightDepth, (inSunSpace.xy + 1) / 2, (inSunSpace.z + 1) / 2);

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