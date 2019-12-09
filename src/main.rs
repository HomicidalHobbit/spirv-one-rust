#![allow(dead_code)]
#![allow(non_snake_case)]
use libc::c_char;
use libc::c_void;
use std::ffi::{CStr, CString};
use std::fs;

use std::ptr::null;

type ShHandle = *const c_void;

#[repr(C)]
struct TLimits {
    nonInductiveForLoops: bool,
    whileLoops: bool,
    doWhileLoops: bool,
    generalUniformIndexing: bool,
    generalAttributeMatrixVectorIndexing: bool,
    generalVaryingIndexing: bool,
    generalSamplerIndexing: bool,
    generalVariableIndexing: bool,
    generalConstantMatrixVectorIndexing: bool,
}

impl Default for TLimits {
    fn default() -> Self {
        TLimits {
            nonInductiveForLoops: true,
            whileLoops: true,
            doWhileLoops: true,
            generalUniformIndexing: true,
            generalAttributeMatrixVectorIndexing: true,
            generalVaryingIndexing: true,
            generalSamplerIndexing: true,
            generalVariableIndexing: true,
            generalConstantMatrixVectorIndexing: true,
        }
    }
}

#[repr(C)]
struct TBuiltInResource {
    maxLights: i32,
    maxClipPlanes: i32,
    maxTextureUnits: i32,
    maxTextureCoords: i32,
    maxVertexAttribs: i32,
    maxVertexUniformComponents: i32,
    maxVaryingFloats: i32,
    maxVertexTextureImageUnits: i32,
    maxCombinedTextureImageUnits: i32,
    maxTextureImageUnits: i32,
    maxFragmentUniformComponents: i32,
    maxDrawBuffers: i32,
    maxVertexUniformVectors: i32,
    maxVaryingVectors: i32,
    maxFragmentUniformVectors: i32,
    maxVertexOutputVectors: i32,
    maxFragmentInputVectors: i32,
    minProgramTexelOffset: i32,
    maxProgramTexelOffset: i32,
    maxClipDistances: i32,
    maxComputeWorkGroupCountX: i32,
    maxComputeWorkGroupCountY: i32,
    maxComputeWorkGroupCountZ: i32,
    maxComputeWorkGroupSizeX: i32,
    maxComputeWorkGroupSizeY: i32,
    maxComputeWorkGroupSizeZ: i32,
    maxComputeUniformComponents: i32,
    maxComputeTextureImageUnits: i32,
    maxComputeImageUniforms: i32,
    maxComputeAtomicCounters: i32,
    maxComputeAtomicCounterBuffers: i32,
    maxVaryingComponents: i32,
    maxVertexOutputComponents: i32,
    maxGeometryInputComponents: i32,
    maxGeometryOutputComponents: i32,
    maxFragmentInputComponents: i32,
    maxImageUnits: i32,
    maxCombinedImageUnitsAndFragmentOutputs: i32,
    maxCombinedShaderOutputResources: i32,
    maxImageSamples: i32,
    maxVertexImageUniforms: i32,
    maxTessControlImageUniforms: i32,
    maxTessEvaluationImageUniforms: i32,
    maxGeometryImageUniforms: i32,
    maxFragmentImageUniforms: i32,
    maxCombinedImageUniforms: i32,
    maxGeometryTextureImageUnits: i32,
    maxGeometryOutputVertices: i32,
    maxGeometryTotalOutputComponents: i32,
    maxGeometryUniformComponents: i32,
    maxGeometryVaryingComponents: i32,
    maxTessControlInputComponents: i32,
    maxTessControlOutputComponents: i32,
    maxTessControlTextureImageUnits: i32,
    maxTessControlUniformComponents: i32,
    maxTessControlTotalOutputComponents: i32,
    maxTessEvaluationInputComponents: i32,
    maxTessEvaluationOutputComponents: i32,
    maxTessEvaluationTextureImageUnits: i32,
    maxTessEvaluationUniformComponents: i32,
    maxTessPatchComponents: i32,
    maxPatchVertices: i32,
    maxTessGenLevel: i32,
    maxViewports: i32,
    maxVertexAtomicCounters: i32,
    maxTessControlAtomicCounters: i32,
    maxTessEvaluationAtomicCounters: i32,
    maxGeometryAtomicCounters: i32,
    maxFragmentAtomicCounters: i32,
    maxCombinedAtomicCounters: i32,
    maxAtomicCounterBindings: i32,
    maxVertexAtomicCounterBuffers: i32,
    maxTessControlAtomicCounterBuffers: i32,
    maxTessEvaluationAtomicCounterBuffers: i32,
    maxGeometryAtomicCounterBuffers: i32,
    maxFragmentAtomicCounterBuffers: i32,
    maxCombinedAtomicCounterBuffers: i32,
    maxAtomicCounterBufferSize: i32,
    maxTransformFeedbackBuffers: i32,
    maxTransformFeedbackInterleavedComponents: i32,
    maxCullDistances: i32,
    maxCombinedClipAndCullDistances: i32,
    maxSamples: i32,
    maxMeshOutputVerticesNV: i32,
    maxMeshOutputPrimitivesNV: i32,
    maxMeshWorkGroupSizeX_NV: i32,
    maxMeshWorkGroupSizeY_NV: i32,
    maxMeshWorkGroupSizeZ_NV: i32,
    maxTaskWorkGroupSizeX_NV: i32,
    maxTaskWorkGroupSizeY_NV: i32,
    maxTaskWorkGroupSizeZ_NV: i32,
    maxMeshViewCountNV: i32,

    limits: TLimits,
}

impl Default for TBuiltInResource {
    fn default() -> Self {
        TBuiltInResource {
            maxLights: 32,
            maxClipPlanes: 6,
            maxTextureUnits: 32,
            maxTextureCoords: 32,
            maxVertexAttribs: 64,
            maxVertexUniformComponents: 4096,
            maxVaryingFloats: 64,
            maxVertexTextureImageUnits: 32,
            maxCombinedTextureImageUnits: 80,
            maxTextureImageUnits: 32,
            maxFragmentUniformComponents: 4096,
            maxDrawBuffers: 32,
            maxVertexUniformVectors: 128,
            maxVaryingVectors: 8,
            maxFragmentUniformVectors: 16,
            maxVertexOutputVectors: 16,
            maxFragmentInputVectors: 15,
            minProgramTexelOffset: -8,
            maxProgramTexelOffset: 7,
            maxClipDistances: 8,
            maxComputeWorkGroupCountX: 65535,
            maxComputeWorkGroupCountY: 65535,
            maxComputeWorkGroupCountZ: 65535,
            maxComputeWorkGroupSizeX: 1024,
            maxComputeWorkGroupSizeY: 1024,
            maxComputeWorkGroupSizeZ: 64,
            maxComputeUniformComponents: 1024,
            maxComputeTextureImageUnits: 16,
            maxComputeImageUniforms: 8,
            maxComputeAtomicCounters: 8,
            maxComputeAtomicCounterBuffers: 1,
            maxVaryingComponents: 60,
            maxVertexOutputComponents: 64,
            maxGeometryInputComponents: 64,
            maxGeometryOutputComponents: 128,
            maxFragmentInputComponents: 128,
            maxImageUnits: 8,
            maxCombinedImageUnitsAndFragmentOutputs: 8,
            maxCombinedShaderOutputResources: 8,
            maxImageSamples: 0,
            maxVertexImageUniforms: 0,
            maxTessControlImageUniforms: 0,
            maxTessEvaluationImageUniforms: 0,
            maxGeometryImageUniforms: 0,
            maxFragmentImageUniforms: 8,
            maxCombinedImageUniforms: 8,
            maxGeometryTextureImageUnits: 16,
            maxGeometryOutputVertices: 256,
            maxGeometryTotalOutputComponents: 1024,
            maxGeometryUniformComponents: 1024,
            maxGeometryVaryingComponents: 64,
            maxTessControlInputComponents: 128,
            maxTessControlOutputComponents: 128,
            maxTessControlTextureImageUnits: 16,
            maxTessControlUniformComponents: 1024,
            maxTessControlTotalOutputComponents: 4096,
            maxTessEvaluationInputComponents: 128,
            maxTessEvaluationOutputComponents: 128,
            maxTessEvaluationTextureImageUnits: 16,
            maxTessEvaluationUniformComponents: 1024,
            maxTessPatchComponents: 120,
            maxPatchVertices: 32,
            maxTessGenLevel: 64,
            maxViewports: 16,
            maxVertexAtomicCounters: 0,
            maxTessControlAtomicCounters: 0,
            maxTessEvaluationAtomicCounters: 0,
            maxGeometryAtomicCounters: 0,
            maxFragmentAtomicCounters: 8,
            maxCombinedAtomicCounters: 8,
            maxAtomicCounterBindings: 1,
            maxVertexAtomicCounterBuffers: 0,
            maxTessControlAtomicCounterBuffers: 0,
            maxTessEvaluationAtomicCounterBuffers: 0,
            maxGeometryAtomicCounterBuffers: 0,
            maxFragmentAtomicCounterBuffers: 1,
            maxCombinedAtomicCounterBuffers: 1,
            maxAtomicCounterBufferSize: 16384,
            maxTransformFeedbackBuffers: 4,
            maxTransformFeedbackInterleavedComponents: 64,
            maxCullDistances: 8,
            maxCombinedClipAndCullDistances: 8,
            maxSamples: 4,
            maxMeshOutputVerticesNV: 256,
            maxMeshOutputPrimitivesNV: 512,
            maxMeshWorkGroupSizeX_NV: 32,
            maxMeshWorkGroupSizeY_NV: 1,
            maxMeshWorkGroupSizeZ_NV: 1,
            maxTaskWorkGroupSizeX_NV: 32,
            maxTaskWorkGroupSizeY_NV: 1,
            maxTaskWorkGroupSizeZ_NV: 1,
            maxMeshViewCountNV: 4,
            limits: Default::default(),
        }
    }
}

const EOPTION_NONE: i32 = 0;
const EOPTION_INTERMEDIATE: i32 = (1 << 0);
const EOPTION_SUPPRESS_INFOLOG: i32 = (1 << 1);
const EOPTION_MEMORY_LEAK_MODE: i32 = (1 << 2);
const EOPTION_RELAXED_ERRORS: i32 = (1 << 3);
const EOPTION_GIVE_WARNINGS: i32 = (1 << 4);
const EOPTION_LINK_PROGRAM: i32 = (1 << 5);
const EOPTION_MULTI_THREADED: i32 = (1 << 6);
const EOPTION_DUMP_CONFIG: i32 = (1 << 7);
const EOPTION_DUMP_REFLECTION: i32 = (1 << 8);
const EOPTION_SUPPRESS_WARNINGS: i32 = (1 << 9);
const EOPTION_DUMP_VERSIONS: i32 = (1 << 10);
const EOPTION_SPV: i32 = (1 << 11);
const EOPTION_HUMAN_READABLE_SPV: i32 = (1 << 12);
const EOPTION_VULKAN_RULES: i32 = (1 << 13);
const EOPTION_DEFAULT_DESKTOP: i32 = (1 << 14);
const EOPTION_OUTPUT_PREPROCESSED: i32 = (1 << 15);
const EOPTION_OUTPUT_HEXADECIMAL: i32 = (1 << 16);
const EOPTION_READ_HLSL: i32 = (1 << 17);
const EOPTION_CASCADING_ERRORS: i32 = (1 << 18);
const EOPTION_AUTO_MAP_BINDINGS: i32 = (1 << 19);
const EOPTION_FLATTEN_UNIFORM_ARRAYS: i32 = (1 << 20);
const EOPTION_NO_STORAGE_FORMAT: i32 = (1 << 21);
const EOPTION_KEEP_UNCALLED: i32 = (1 << 22);
const EOPTION_HLSL_OFFSETS: i32 = (1 << 23);
const EOPTION_HLSL_IO_MAPPING: i32 = (1 << 24);
const EOPTION_AUTO_MAP_LOCATIONS: i32 = (1 << 25);
const EOPTION_DEBUG: i32 = (1 << 26);
const EOPTION_STDIN: i32 = (1 << 27);
const EOPTION_OPTIMIZE_DISABLE: i32 = (1 << 28);
const EOPTION_OPTIMIZE_SIZE: i32 = (1 << 29);
const EOPTION_INVERT_Y: i32 = (1 << 30);
const EOPTION_DUMP_BARE_VERSION: i32 = (1 << 31);

#[repr(C)]
enum EShLanguage {
    EShLangVertex,
    EShLangTessControl,
    EShLangTessEvaluation,
    EShLangGeometry,
    EShLangFragment,
    EShLangCompute,
    EShLangRayGenNV,
    EShLangIntersectNV,
    EShLangAnyHitNV,
    EShLangClosestHitNV,
    EShLangMissNV,
    EShLangCallableNV,
    EShLangTaskNV,
    EShLangMeshNV,
    EShLangCount,
}

#[repr(C)]
enum EShOptimizationLevel {
    EShOptNoGeneration,
    EShOptNone,
    EShOptSimple, // Optimizations that can be done quickly
    EShOptFull,   // Optimizations that will take more time
}

const ESH_MSG_DEFAULT: i32 = 0; // default is to give all required errors and extra warnings
const ESH_MSG_RELAXED_ERRORS: i32 = (1 << 0); // be liberal in accepting input
const ESH_MSG_SUPPRESS_WARNINGS: i32 = (1 << 1); // suppress all warnings, except those required by the specification
const ESH_MSG_AST: i32 = (1 << 2); // print the AST intermediate representation
const ESH_MSG_SPV_RULES: i32 = (1 << 3); // issue messages for SPIR-V generation
const ESH_MSG_VULKAN_RULES: i32 = (1 << 4); // issue messages for Vulkan-requirements of GLSL for SPIR-V
const ESH_MSG_ONLY_PREPROCESSOR: i32 = (1 << 5); // only print out errors produced by the preprocessor
const ESH_MSG_READ_HLSL: i32 = (1 << 6); // use HLSL parsing rules and semantics
const ESH_MSG_CASCADING_ERRORS: i32 = (1 << 7); // get cascading errors; risks error-recovery issues, instead of an early exit
const ESH_MSG_KEEP_UNCALLED: i32 = (1 << 8); // for testing, don't eliminate uncalled functions
const ESH_MSG_HLSL_OFFSETS: i32 = (1 << 9); // allow block offsets to follow HLSL rules instead of GLSL rules
const ESH_MSG_DEBUG_INFO: i32 = (1 << 10); // save debug information
const ESH_MSG_HLSL_ENABLE16_BIT_TYPES: i32 = (1 << 11); // enable use of 16-bit types in SPIR-V for HLSL
const ESH_MSG_HLSL_LEGALIZATION: i32 = (1 << 12); // enable HLSL Legalization messages
const ESH_MSG_HLSL_DX9_COMPATIBLE: i32 = (1 << 13); // enable HLSL DX9 compatible mode (right now only for samplers)
const ESH_MSG_BUILTIN_SYMBOL_TABLE: i32 = (1 << 14); // print the builtin symbol table

#[repr(C)]
enum EShExecutable {
    EShExVertexFragment,
    EShExFragment,
}

#[link(name = "glslang", kind = "static")]

extern "C" {
    fn ShInitialize() -> i32;

    fn ShFinalize() -> i32;

    fn ShCompile(
        sh_handle: ShHandle,
        shader_strings: *const *const c_char,
        num_strings: i32,
        lengths: *const i32,
        optimization_level: EShOptimizationLevel,
        t_builtin_resource: *const TBuiltInResource,
        debug_options: i32,
        default_version: i32,
        forward_compatible: bool,
        messages: i32,
    ) -> i32;

    fn ShConstructCompiler(stage: EShLanguage, debug_options: i32) -> ShHandle;

    fn ShDestruct(sh_handle: ShHandle);

    fn ShLinkExt(sh_handle: ShHandle, handles: *const ShHandle, num_handles: i32) -> i32;

    fn ShConstructLinker(executable: EShExecutable, debug_optiions: i32) -> ShHandle;

    fn ShGetInfoLog(handle: ShHandle) -> *const c_char;

}

#[link(name = "HLSL", kind = "static")]
extern "C" {}

#[link(name = "OSDependent", kind = "static")]
extern "C" {}

#[link(name = "OGLCompiler", kind = "static")]
extern "C" {}

fn main() {
    let resource: TBuiltInResource = Default::default();
    let mut compilers: Vec<ShHandle> = Vec::new();
    let linker: ShHandle;
    unsafe {
        ShInitialize();
        linker = ShConstructLinker(EShExecutable::EShExVertexFragment, EOPTION_NONE);
        if linker == null() {
            panic!("Cannot locate correct linker!");
        }
    }

    compilers.push(compile_shader(
        "shader.vert.glsl",
        EShLanguage::EShLangVertex,
        EOPTION_AUTO_MAP_LOCATIONS | EOPTION_HUMAN_READABLE_SPV | EOPTION_LINK_PROGRAM,
        &resource,
    ));

    compilers.push(compile_shader(
        "shader.frag.glsl",
        EShLanguage::EShLangFragment,
        EOPTION_AUTO_MAP_LOCATIONS | EOPTION_HUMAN_READABLE_SPV | EOPTION_LINK_PROGRAM,
        &resource,
    ));

    unsafe {
        let ret = ShLinkExt(linker, compilers.as_ptr(), compilers.len() as i32);
        let result_str = CStr::from_ptr(ShGetInfoLog(linker));
        let result = result_str.to_str().unwrap();
        if ret == 0 {
            println!("Error: Linker Failed!\n{}", result);
        }

        ShDestruct(linker);
        for compiler in compilers {
            ShDestruct(compiler);
        }
        ShFinalize();
    }
}

fn compile_shader(
    name: &str,
    stage: EShLanguage,
    options: i32,
    resource: &TBuiltInResource,
) -> ShHandle {
    let ret;
    let source = fs::read_to_string(name).unwrap();
    print!("Compiling Shader: '{}' ", name);
    let csource = CString::new(source).expect("Failed!");
    let chars = csource.as_bytes();

    let zchars = Vec::from(chars);
    let mut strings: Vec<*const u8> = Vec::new();
    strings.push(zchars.as_ptr());

    let mut lengths: Vec<i32> = Vec::new();
    lengths.push(chars.len() as i32);

    let compiler: ShHandle;
    let result_str: &CStr;
    unsafe {
        compiler = ShConstructCompiler(stage, options);
        if compiler == null() {
            panic!("Cannot locate correct compiler!");
        }

        ret = ShCompile(
            compiler,
            strings.as_ptr() as _,
            lengths.len() as i32,
            lengths.as_ptr(),
            EShOptimizationLevel::EShOptNone,
            resource,
            1,
            110,
            false,
            ESH_MSG_SPV_RULES | ESH_MSG_VULKAN_RULES | ESH_MSG_AST,
        );

        result_str = CStr::from_ptr(ShGetInfoLog(compiler));
    }

    let result = result_str.to_str().unwrap();

    if ret == 0 {
        println!("Failed!\n{}", result);
    } else {
        println!("OK!\n{}", result)
    }
    compiler
}
