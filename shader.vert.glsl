#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTex;

uniform mat4 viewProjectionMatrix;
uniform mat4 modelMatrix;

out vec2 texCoord;

void main()
{
	texCoord = aTex;
	gl_Position = viewProjectionMatrix * modelMatrix * vec4(aPos, 1.0);
}
