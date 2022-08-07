#version 450

layout(location = 0) out vec4 f_color;
layout(location = 1) in vec2 pos;

void main() {
    float err = abs(distance(pos, vec2(0, pow(pos.x * pos.x, 1.0 / 3.0))) - 1.0);
    f_color = vec4(1.0, err, err, 1.0);
}
