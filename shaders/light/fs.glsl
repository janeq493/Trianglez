#version 330 core
in vec3 col_o;
out vec4 FragColor;

uniform vec3 light_source;
uniform vec3 light_color;

void main()
{
    float ambient_str = 0.1;
    FragColor = vec4(ambient_str*col_o,1.0f);
}
