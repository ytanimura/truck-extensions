use glsl_to_spirv::ShaderType;
use std::io::Read;

pub fn compile_glsl(code: &str, shadertype: ShaderType) -> Vec<u8> {
    let mut spirv = glsl_to_spirv::compile(&code, shadertype).unwrap();
    let mut compiled = Vec::new();
    spirv.read_to_end(&mut compiled).unwrap();
    compiled
}
