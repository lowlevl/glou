#version 330

void main() {
    if (gl_FragCoord.x < 300.0 && gl_FragCoord.y < 300.0) {
        gl_FragColor = vec4(0.0, 0.0, 1.0, 1.0);
    }

    if (gl_FragCoord.x > 200.0 && gl_FragCoord.y > 200.0) {
        gl_FragColor = vec4(0.0, 1.0, 0.0, 1.0);
    }

    if (gl_FragCoord.x == gl_FragCoord.y) {
        gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
    }
}
