#version 450
layout (location = 3) out vec4 FragColour;
layout (location = 4) in vec2 texCoord;

layout (binding = 5) uniform sampler2D ourTexture;

void main()
{
	vec4 colour = texture(ourTexture, texCoord); 
	FragColour = vec4(colour.xyz, 1.0);
}
