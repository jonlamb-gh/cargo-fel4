use std::fs;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

use super::{run_cmd, Error};
use config::Config;

/// Create a new feL4 project.
///
/// Generates all of the scaffolding files for a new
/// feL4 project.
pub fn handle_new_cmd(config: &Config) -> Result<(), Error> {
    // Default project name is hard coded until we replace
    // our option handling mechanism
    // TODO - use config/cli options
    let package_name = "fel4-project";

    generate_baseline_cargo_package(config)?;

    generate_fel4_project_files(config)?;

    generate_target_specs(config)?;

    Ok(())
}

fn generate_baseline_cargo_package(config: &Config) -> Result<(), Error> {
    // Default project name is hard coded until we replace
    // our option handling mechanism
    // TODO - use config/cli options
    let package_name = "fel4-project";

    let mut cmd = Command::new("cargo");
    cmd.arg("new");

    // We passthrough these to cargo
    if config.cli_args.flag_verbose {
        cmd.arg("--verbose");
    } else if config.cli_args.flag_quiet {
        cmd.arg("--quiet");
    }

    // The project name and directory are the same
    cmd.arg("--name").arg(&package_name);

    run_cmd(cmd.arg("--lib").arg(&package_name))?;

    Ok(())
}

fn generate_fel4_project_files(config: &Config) -> Result<(), Error> {
    // Default project name is hard coded until we replace
    // our option handling mechanism
    // TODO - use config/cli options
    let package_name = "fel4-project";

    // Create example feL4 application thread run function
    let mut lib_src_file = File::create(Path::new(package_name).join("src").join("lib.rs"))?;
    lib_src_file.write_all(APP_LIB_CODE.as_bytes())?;

    // Add feL4 dependencies to Cargo.toml
    let mut cargo_toml_file = OpenOptions::new()
        .append(true)
        .open(Path::new(package_name).join("Cargo.toml"))?;
    cargo_toml_file.write_all(
        b"libsel4-sys = {git = \"ssh://github.com/PolySync/fel4-dependencies.git\", branch =           \"devel\"}",
    )?;

    let mut fel4_toml_file = File::create(Path::new(package_name).join("fel4.toml"))?;
    fel4_toml_file.write_all(FEL4_TOML_TEXT.as_bytes())?;

    // Create Xargo.toml with our target features
    let mut xargo_toml_file = File::create(Path::new(package_name).join("Xargo.toml"))?;
    xargo_toml_file.write_all(XARGO_TOML_TEXT.as_bytes())?;

    Ok(())
}

fn generate_target_specs(config: &Config) -> Result<(), Error> {
    // Default project name is hard coded until we replace
    // our option handling mechanism
    // TODO - use config/cli options
    let package_name = "fel4-project";

    // Create target specifications directory and specification files
    let target_specs_path = Path::new(package_name).join("target_specs");
    fs::create_dir(Path::new(&target_specs_path))?;

    let mut target_spec_readme_file = File::create(&target_specs_path.join("README.md"))?;
    target_spec_readme_file.write_all(FEL4_TARGET_SPEC_README.as_bytes())?;

    let mut target_spec_x86_64_file =
        File::create(&target_specs_path.join("x86_64-sel4-fel4.json"))?;
    target_spec_x86_64_file.write_all(FEL4_TARGET_SPEC_X86_64_SEL4_FEL4.as_bytes())?;

    let mut target_spec_arm_file = File::create(&target_specs_path.join("arm-sel4-fel4.json"))?;
    target_spec_arm_file.write_all(FEL4_TARGET_SPEC_ARM_SEL4_FEL4.as_bytes())?;

    Ok(())
}

const FEL4_TARGET_SPEC_README: &'static str = include_str!("../target_specs/README.md");

const FEL4_TARGET_SPEC_X86_64_SEL4_FEL4: &'static str =
    include_str!("../target_specs/x86_64-sel4-fel4.json");

const FEL4_TARGET_SPEC_ARM_SEL4_FEL4: &'static str =
    include_str!("../target_specs/arm-sel4-fel4.json");

const FEL4_TOML_TEXT: &'static str = include_str!("../configs/fel4.toml");

const APP_LIB_CODE: &'static str = include_str!("../templates/lib.rs");

const XARGO_TOML_TEXT: &'static str = include_str!("../templates/Xargo.toml");
