#version 450

layout(location = 0) in vec2 position;
layout(location = 1) out vec2 pos;

void main() {
    pos = 3.0 * vec2(position.x, -position.y);
    gl_Position = vec4(position, 0.0, 1.0);
}
