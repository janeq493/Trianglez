#version 330 core
layout (location=0) in vec3 pos;
layout (location=1) in vec3 col;

out vec3 col_o;

uniform mat4 mod_mat;
uniform mat4 view_mat;
uniform mat4 proj_mat;

void main()
{
    gl_Position = proj_mat * view_mat * mod_mat * vec4(pos.xyz,1);
    col_o=col;
}
