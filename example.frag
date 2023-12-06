#version 330

precision highp float;

uniform vec2 u_resolution;
uniform float u_time;

void main() {
    vec2 uv = gl_FragCoord.xy / u_resolution;

    vec3 color = vec3(uv.x, uv.y, abs(cos(u_time)));

    if (int(uv.x + uv.y) == 1)
        color = vec3(1.0);

    gl_FragColor = vec4(color, 1.0);
}
