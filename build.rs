use std::path::PathBuf;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let shader_crate = PathBuf::from("./shaders");
    let backend = cargo_gpu::Install::from_shader_crate(shader_crate.clone()).run()?;
    let builder = backend.to_spirv_builder(shader_crate, "spirv-unknown-vulkan1.2");
    let _compile_result = builder.build()?;
    Ok(())
}
