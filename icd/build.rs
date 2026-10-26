//! Generates the ICD dispatch tables and stubs for unimplemented dispatched
//! commands, and enables nightly-only debug helpers.

use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    generate_dispatch_tables()?;
    enable_wait_for_debugger();
    Ok(())
}

fn generate_dispatch_tables() -> Result<(), Box<dyn Error>> {
    let vk_xml_path = codegen::vk_xml_path();
    let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    println!("cargo::rerun-if-changed={}", vk_xml_path.display());
    println!("cargo::rerun-if-changed={}", src_dir.display());
    println!("cargo::rerun-if-changed=../codegen/src/");
    println!("cargo::rerun-if-changed=build.rs");

    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").expect("Cargo sets OUT_DIR"));
    let vk_xml = codegen::VkXml::from(&vk_xml_path)?;
    let implemented = implemented_commands(&src_dir)?;
    let extensions: Vec<&str> = codegen::policy::INSTANCE_EXTENSIONS
        .iter()
        .chain(codegen::policy::DEVICE_EXTENSIONS)
        .copied()
        .collect();
    let options = codegen::DispatchOptions {
        extensions: &extensions,
        loader_commands: codegen::policy::LOADER_COMMANDS,
        instance_commands: codegen::policy::INSTANCE_COMMANDS,
    };

    let mut generated = Vec::new();
    vk_xml.write_dispatch_tables(&mut generated, &options, &implemented)?;
    std::fs::write(out_dir.join("dispatch_tables.rs"), generated)?;
    Ok(())
}

fn implemented_commands(dir: &Path) -> io::Result<HashSet<String>> {
    fn visit(dir: &Path, commands: &mut HashSet<String>) -> io::Result<()> {
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_dir() {
                visit(&path, commands)?;
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                let source = fs::read_to_string(&path)?;
                for line in source.lines() {
                    let Some(rest) = line
                        .trim_start()
                        .strip_prefix("pub unsafe extern \"C\" fn ")
                    else {
                        continue;
                    };
                    let name: String = rest
                        .chars()
                        .take_while(|character| {
                            character.is_ascii_alphanumeric() || *character == '_'
                        })
                        .collect();
                    if name.starts_with("vk") {
                        commands.insert(name);
                    }
                }
            }
        }
        Ok(())
    }

    let mut commands = HashSet::new();
    visit(dir, &mut commands)?;
    Ok(commands)
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
