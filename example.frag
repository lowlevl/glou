#version 330

void main() {
    if (gl_FragCoord.x < 200.0 && gl_FragCoord.y < 200.0) {
        gl_FragColor = vec4(1.0);
    }
}
