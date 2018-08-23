#version 140

in vec2 position;

uniform mat3 perspective;

void main() {
     gl_Position = vec4(perspective * vec3(position, 1.0), 1.0);
}