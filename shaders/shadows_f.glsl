#version 140

uniform sampler2D sunDepth;
uniform sampler2D sunDistantDepth;
uniform sampler2D cameraDepth;
uniform sampler2D cameraColor;
uniform sampler2D cameraNormals;

uniform vec3 sunPosition;

uniform mat4 sunProjection;
uniform mat4 sunDistantProjection;
uniform mat4 sunView;
uniform mat4 cameraProjection;
uniform mat4 cameraView;

in vec2 Texcoord;
out vec4 color;


float shadowing(sampler2D shadowMap, vec2 projCoords, float currentDepth) {
    float shadow = 0.0;
    float bias = 0.0004;
    int samples = 1;
    int samplesTaken = 0;
    vec2 texelSize = 1.0 / textureSize(shadowMap, 0);
    for(int x = 0 - samples; x <= samples; ++x)
    {
        for(int y = 0 - samples; y <= samples; ++y)
        {
            samplesTaken += 1;
            float pcfDepth = texture(shadowMap, projCoords.xy + vec2(x, y) * texelSize).r; 
            shadow += currentDepth - bias > pcfDepth ? 0.3 : 1.0;        
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

    vec3 directionToSun = normalize(sunPosition - inWorldSpace.xyz);
    vec3 normal = normalize(texture(cameraNormals, Texcoord).xyz * 2 - 1);
    float difference = dot(directionToSun, normal);

    vec4 inSunSpace = sunProjection * sunView * vec4(inWorldSpace.xyz, 1.0);
    vec4 inDistantSunSpace = sunDistantProjection * sunView * vec4(inWorldSpace.xyz, 1.0);

    float acDistance = 0;

    float nearShadowing = shadowing(sunDepth, (inSunSpace.xy + 1) / 2, (inSunSpace.z + 1) / 2);
    float farShadowing = shadowing(sunDistantDepth, (inDistantSunSpace.xy + 1) / 2, (inDistantSunSpace.z + 1) / 2);

   

    acDistance = nearShadowing;

    if (inSunSpace.x > 1 || inSunSpace.x < -1 || inSunSpace.y > 1 || inSunSpace.y < -1) {
            acDistance = farShadowing;
    }

    if (difference < 0.0) {
        acDistance = 0.3;
    }

    vec4 cameraColorVal = texture(cameraColor, Texcoord);

  //  float ao = getAO();

    vec3 cameraColorValShadowed = mix(cameraColorVal.rgb, cameraColorVal.rgb * acDistance, 1);
//    cameraColorValShadowed = mix(cameraColorValShadowed, cameraColorValShadowed * ao, 0.5);


    color = vec4(cameraColorValShadowed, 1.0);
}