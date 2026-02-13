use cargo_gpu_install::install::Install;
use std::path::PathBuf;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::builder().init();

    let shader_crate = PathBuf::from("./shaders");

    // install the toolchain and build the `rustc_codegen_spirv` codegen backend with it
    let backend = Install::from_shader_crate(shader_crate.clone()).run()?;

    // build the shader crate
    let mut builder = backend.to_spirv_builder(shader_crate, "spirv-unknown-naga-wgsl");
    builder.build_script.defaults = true;
    builder.build_script.env_shader_spv_path = Some(true);
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
