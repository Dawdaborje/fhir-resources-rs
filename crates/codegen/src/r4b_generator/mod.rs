//! FHIR R4B Code Generator
//!
//! Generates Rust types from FHIR R4B schema definitions.

use anyhow::{Context, Result};
use std::path::Path;

mod parser;

/// Generate Rust types from FHIR R4B schemas
pub fn generate() -> Result<()> {
    println!("  Parsing FHIR R4B schemas...");

    // Define paths - R4B schemas are in definitions.json/ subdirectory
    let schema_dir = Path::new("schemas/r4b/definitions.json");
    let output_dir = Path::new("crates/resources/src/r4b");

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

    let type_code = crate::common::codegen::generate_types(&types, "r4b")?;
    let resource_files = crate::common::codegen::generate_resources(&resources, "r4b")?;

    println!("    Generated {} resource files", resource_files.len());

    // Write files
    println!("  Writing files...");

    crate::common::writer::write_module(output_dir, type_code, resource_files, "r4b")
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
