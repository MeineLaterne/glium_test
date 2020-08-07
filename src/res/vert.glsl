#version 140
in vec2 position;
uniform mat4 transform;
uniform mat4 projection;
void main() {
    gl_Position = projection * transform * vec4(position, 0.0, 1.0);
}