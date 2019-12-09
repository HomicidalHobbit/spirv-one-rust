#version 420

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTex;

layout(binding = 0) uniform Matrices
{
 	mat4 viewProjectionMatrix;
	mat4 modelMatrix;
};

layout (location = 0) out vec2 texCoord;

void main()
{
	texCoord = aTex;
	gl_Position = viewProjectionMatrix * modelMatrix * vec4(aPos, 1.0);
}
