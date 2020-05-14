#version 140
in vec3 Normal;
in vec3 Color;
in vec4 SunPosition;
in vec4 SunPosition2;

out vec4 color;

uniform sampler2D sunDepth;
uniform sampler2D sunDepth2;

float getShadow(vec3 projCoords, sampler2D text) {
    float sunDepthAtPos = texture(text, projCoords.xy).r;

    float shadow = 0.0;
    float bias = 0.001;
    vec2 texelSize = 1.0 / textureSize(text, 0);
    for(int x = -1; x <= 1; ++x)
    {
        for(int y = -1; y <= 1; ++y)
        {
            float pcfDepth = texture(text, projCoords.xy + vec2(x, y) * texelSize).r;
            float dist = distance(projCoords.z, pcfDepth);
            if (projCoords.z - 0.002 > pcfDepth) {
                shadow += 0.5;
            } else {
                shadow += 1;
            }
        }
    }
    shadow /= 9;
    return shadow;
}

void main() {
    float brightness = dot(normalize(Normal), normalize(vec3(50.0, -70.0, 30.0)));
    vec3 reg_color = vec3(Color.x / 255, Color.y / 255, Color.z / 255);
    vec3 dark_color = reg_color / 2;
    vec3 unshadowed = mix(dark_color, reg_color, brightness);
    unshadowed = reg_color;

    vec3 projCoords = SunPosition.xyz / SunPosition.w;
    projCoords = projCoords * 0.5 + 0.5;

    float shadow = 0.0;

    if (projCoords.x > 1 || projCoords.x < 0 || projCoords.y > 1 || projCoords.y < 0) {
        vec3 projCoords2 = SunPosition2.xyz / SunPosition2.w;
        projCoords2 = projCoords2 * 0.5 + 0.5;

        if (projCoords2.x > 1 || projCoords2.x < 0 || projCoords2.y > 1 || projCoords2.y < 0) {
            shadow = 1.0;
        } else {
            shadow = getShadow(projCoords2, sunDepth2);
        }
    } else {
        shadow = getShadow(projCoords, sunDepth);
    }

    color = vec4(unshadowed.rgb * shadow, 1.0);
}