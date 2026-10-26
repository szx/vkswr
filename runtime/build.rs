//! Generates the extension properties advertised by this ICD.

use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let vk_xml_path = codegen::vk_xml_path();
    println!("cargo::rerun-if-changed={}", vk_xml_path.display());
    println!("cargo::rerun-if-changed=../codegen/src/");
    println!("cargo::rerun-if-changed=build.rs");

    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").expect("Cargo sets OUT_DIR"));
    let vk_xml = codegen::VkXml::from(&vk_xml_path)?;

    let mut generated = Vec::new();
    vk_xml.write_advertised_extensions(
        &mut generated,
        codegen::policy::INSTANCE_EXTENSIONS,
        codegen::policy::DEVICE_EXTENSIONS,
    )?;
    std::fs::write(out_dir.join("advertised_extensions.rs"), generated)?;
    Ok(())
}
