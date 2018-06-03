#version 330 core
in vec3 col_o;
out vec4 FragColor;


void main()
{
    FragColor = vec4(col_o,1.0f);
}
