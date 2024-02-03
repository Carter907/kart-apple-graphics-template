#version 330
in vec3 pos;
uniform mat4 view;
uniform mat4 model;
uniform mat4 proj;
in vec3 vert_color;
out vec3 color;
void main() {
    // multiplication is not communicative for matrices!
    gl_Position = proj * view * model * vec4(pos, 1.0);

    color = vert_color;
}
