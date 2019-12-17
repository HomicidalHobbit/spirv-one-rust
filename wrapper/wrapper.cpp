#include "wrapper.h"

void InitializeProcess()
{
	glslang::InitializeProcess();
}

void FinalizeProcess()
{
	glslang::FinalizeProcess();
}

glslang::TProgram* create_program()
{
	return nullptr;
}