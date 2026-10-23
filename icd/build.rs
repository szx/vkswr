//! Generates the ICD dispatch tables and enables nightly-only debug helpers.

use std::error::Error;
use std::path::PathBuf;

/// Extensions advertised by the ICD. Every command they add is dispatched.
///
/// Keep this in sync with the extension properties reported by the `runtime`
/// crate.
const ADVERTISED_EXTENSIONS: &[&str] = &[
    "VK_KHR_surface",
    "VK_KHR_xcb_surface",
    "VK_KHR_swapchain",
    "VK_EXT_tooling_info",
    "VK_EXT_debug_marker",
];

/// Global commands implemented by the Vulkan loader instead of the ICD.
const LOADER_COMMANDS: &[&str] = &[
    "vkGetInstanceProcAddr",
    "vkEnumerateInstanceLayerProperties",
    "vkEnumerateDeviceLayerProperties",
];

fn main() -> Result<(), Box<dyn Error>> {
    generate_dispatch_tables()?;
    enable_wait_for_debugger();
    Ok(())
}

fn generate_dispatch_tables() -> Result<(), Box<dyn Error>> {
    println!("cargo::rerun-if-changed=../codegen/vk.xml");
    println!("cargo::rerun-if-changed=../codegen/src/");
    println!("cargo::rerun-if-changed=build.rs");

    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").expect("Cargo sets OUT_DIR"));
    let vk_xml = codegen::VkXml::from("../codegen/vk.xml")?;
    let options = codegen::DispatchOptions {
        extensions: ADVERTISED_EXTENSIONS,
        loader_commands: LOADER_COMMANDS,
        // The loader resolves vkGetDeviceProcAddr through vk_icdGetInstanceProcAddr.
        instance_commands: &["vkGetDeviceProcAddr"],
    };

    let mut generated = Vec::new();
    vk_xml.write_dispatch_tables(&mut generated, &options)?;
    std::fs::write(out_dir.join("dispatch_tables.rs"), generated)?;
    Ok(())
}

#[rustversion::nightly]
fn enable_wait_for_debugger() {
    println!("cargo::rustc-cfg=wait_for_debugger");
    println!("cargo::rustc-check-cfg=cfg(wait_for_debugger)");
}

#[rustversion::not(nightly)]
fn enable_wait_for_debugger() {
    println!("cargo::rustc-check-cfg=cfg(wait_for_debugger)");
}
