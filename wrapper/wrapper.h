#pragma once
#include <Public/ShaderLang.h>

extern "C" {
	void InitializeProcess();
	void FinalizeProcess();
	
	glslang::TProgram* CreateProgram();
	glslang::TShader* CreateShader(EShLanguage stage);

	void DestroyProgram(glslang::TProgram*);
	void DestroyShader(glslang::TShader*);
	
    void SetAutoMapBindings(glslang::TShader*, bool);
    void SetAutoMapLocations(glslang::TShader*, bool);

	void SetShiftBinding(glslang::TShader* shader, glslang::TResourceType res, unsigned int base);
	void SetStrings(glslang::TShader* shader, const char* const* s, int n);
	void SetStringsWithLengths(glslang::TShader* shader, const char* const* s, const int* l, int n);
	void SetStringsWithLengthsAndNames(glslang::TShader* shader, const char* const* s, const int* l, const char* const* names, int n);
}