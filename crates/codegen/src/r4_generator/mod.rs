//! FHIR R4 Code Generator
//!
//! Generates Rust types from FHIR R4 schema definitions.

use anyhow::{Context, Result};
use std::path::Path;

mod parser;

/// Generate Rust types from FHIR R4 schemas
pub fn generate() -> Result<()> {
    println!("  Parsing FHIR R4 schemas...");

    // Define paths - these are relative to workspace root
    let schema_dir = Path::new("schemas/r4");
    let output_dir = Path::new("crates/resources/src/r4");

    // Parse schemas
    let resources = parser::parse_bundle(&schema_dir.join("profiles-resources.json"))
        .context("Failed to parse profiles-resources.json")?;

    let types = parser::parse_bundle(&schema_dir.join("profiles-types.json"))
        .context("Failed to parse profiles-types.json")?;

    println!(
        "    Parsed {} resources and {} types",
        resources.len(),
        types.len()
    );

    // Generate code
    println!("  Generating Rust code...");

    let type_code = crate::common::codegen::generate_types(&types, "r4")?;
    let resource_files = crate::common::codegen::generate_resources(&resources, "r4")?;

    println!("    Generated {} resource files", resource_files.len());

    // Write files
    println!("  Writing files...");

    crate::common::writer::write_module(output_dir, type_code, resource_files, "r4")
        .context("Failed to write generated files")?;

    // Format code
    println!("  Formatting code with rustfmt...");

    match crate::common::writer::format_code(output_dir) {
        Ok(_) => println!("    Code formatted successfully"),
        Err(e) => println!(
            "    Warning: rustfmt failed ({}), code may be unformatted",
            e
        ),
    }

    Ok(())
}
