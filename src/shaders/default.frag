#version 150 core

uniform vec3 iResolution;

in vec2 fragCoord;
out vec4 fragColor;

void mainImage(out vec4 fragColor, in vec2 fragCoord) {
    vec2 uv = fragCoord.xy / iResolution.xy;

    fragColor = vec4(uv, 0.5 + 0.5*sin(0.1), 1.0);
}

void main() {
  mainImage(fragColor, fragCoord);
}
