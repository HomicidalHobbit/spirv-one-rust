#version 330 core
out vec4 FragColour;
in vec2 texCoord;

uniform sampler2D ourTexture;

void main()
{
	vec4 colour = texture(ourTexture, texCoord); 
	FragColour = vec4(colour.xyz, 1.0);
}
