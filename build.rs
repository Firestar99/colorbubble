use cargo_gpu::spirv_builder::{MetadataPrintout, SpirvMetadata};
use std::path::PathBuf;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::builder().init();

    let shader_crate = PathBuf::from("./shaders");

    // install the toolchain and build the `rustc_codegen_spirv` codegen backend with it
    let backend = cargo_gpu::Install::from_shader_crate(shader_crate.clone()).run()?;

    // build the shader crate
    let mut builder = backend.to_spirv_builder(shader_crate, "spirv-unknown-naga-wgsl");
    builder.print_metadata = MetadataPrintout::DependencyOnly;
    builder.spirv_metadata = SpirvMetadata::Full;
    let wgsl_result = builder.build()?;
    let path_to_spv = wgsl_result.module.unwrap_single();

    // needs to be fixed upstream
    let path_to_wgsl = path_to_spv.with_extension("wgsl");

    // emit path to wgsl into env var, used in `quad.rs` like this:
    // > include_str!(env!("COLORBUBBLE_WGSL_SHADER_PATH"))
    println!(
        "cargo::rustc-env=COLORBUBBLE_WGSL_SHADER_PATH={}",
        path_to_wgsl.display()
    );

    // you could also generate some rust source code into the `std::env::var("OUT_DIR")` dir
    // and use `include!(concat!(env!("OUT_DIR"), "/shader_symbols.rs"));` to include it
    Ok(())
}
