uniform vec2 u_resolution;
uniform vec2 u_mouse;
uniform float u_time;

void main() {
    vec2 uv = gl_FragCoord.xy / u_resolution;
    float aspect = u_resolution.x / u_resolution.y;
    uv.x *= aspect;

    vec2 mouse = u_mouse / u_resolution;
    mouse.x *= aspect;

    vec3 color = vec3(uv.x, uv.y, abs(cos(u_time)));

    if (uv.x + uv.y >= 0.95 && uv.x + uv.y <= 1.05 )
        color = vec3(1.0);

    if (length(uv - mouse) <= 0.05)
        color = vec3(0.0);

    gl_FragColor = vec4(color, 1.0);
}
