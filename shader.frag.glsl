#version 420
layout (location = 0) out vec4 FragColour;
layout (location = 0) in vec2 texCoord;

layout (binding = 0) uniform sampler2D ourTexture;

void main()
{
	vec4 colour = texture(ourTexture, texCoord); 
	FragColour = vec4(colour.xyz, 1.0);
}
