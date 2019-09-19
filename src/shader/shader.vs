 #version 140

in vec2 position;
out vec3 vvv;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    vvv = vec3(position, 1.0);
}