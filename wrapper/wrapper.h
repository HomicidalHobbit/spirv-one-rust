#pragma once
#include <Public/ShaderLang.h>

extern "C" {
	void InitializeProcess();
	void FinalizeProcess();
	glslang::TProgram* CreateProgram();
}