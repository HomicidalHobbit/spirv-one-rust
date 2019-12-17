#include "wrapper.h"

void InitializeProcess()
{
	glslang::InitializeProcess();
}

void FinalizeProcess()
{
	glslang::FinalizeProcess();
}

glslang::TProgram* CreateProgram()
{
	return new glslang::TProgram();
}

glslang::TShader* CreateShader(EShLanguage stage)
{
	return new glslang::TShader(stage);
}

void DestroyProgram(glslang::TProgram* program)
{
	program->~TProgram();
}

void DestroyShader(glslang::TShader* shader)
{
	shader->~TShader();
}

void SetShiftBinding(glslang::TShader* shader, glslang::TResourceType res, unsigned int base)
{
	shader->setShiftBinding(res, base);
}

void SetStrings(glslang::TShader* shader, const char* const* s, int n)
{
	shader->setStrings(s, n);
}

void SetStringsWithLengths(glslang::TShader* shader, const char* const* s, const int* l, int n)
{
	shader->setStringsWithLengths(s, l, n);
}

void SetStringsWithLengthsAndNames(glslang::TShader* shader, const char* const* s, const int* l, const char* const* names, int n)
{
	shader->setStringsWithLengthsAndNames(s, l, names, n);
}