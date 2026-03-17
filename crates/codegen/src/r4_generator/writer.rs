//! File Writing and Formatting
//!
//! Writes generated Rust code to files and invokes rustfmt.

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;

/// Write module files to output directory
pub fn write_module(
    output_dir: &Path,
    types_content: String,
    resource_files: HashMap<String, String>,
) -> Result<()> {
    // Create output directory
    fs::create_dir_all(output_dir)
        .with_context(|| format!("Failed to create directory: {}", output_dir.display()))?;

    // Write types.rs
    let types_path = output_dir.join("types.rs");
    write_file(&types_path, &types_content)?;

    // Write resource files
    for (file_name, content) in &resource_files {
        let file_path = output_dir.join(format!("{}.rs", file_name));
        write_file(&file_path, content)?;
    }

    // Generate mod.rs
    create_mod_file(output_dir, &resource_files)?;

    Ok(())
}

/// Write content to a file
pub fn write_file(path: &Path, content: &str) -> Result<()> {
    fs::write(path, content)
        .with_context(|| format!("Failed to write file: {}", path.display()))?;
    Ok(())
}

/// Create mod.rs with module declarations
fn create_mod_file(dir: &Path, resource_files: &HashMap<String, String>) -> Result<()> {
    let mut content = String::new();

    content.push_str("//! FHIR R4 Resources and Types\n");
    content.push_str("//!\n");
    content.push_str("//! Auto-generated from FHIR schema definitions.\n");
    content.push_str("//! Do not modify directly.\n\n");

    // Always include types
    content.push_str("pub mod types;\n\n");

    // Sort and add resource modules
    let mut module_names: Vec<_> = resource_files.keys().collect();
    module_names.sort();

    for module_name in module_names {
        content.push_str(&format!("pub mod {};\n", module_name));
    }

    let mod_path = dir.join("mod.rs");
    write_file(&mod_path, &content)?;

    Ok(())
}

/// Format code using rustfmt
pub fn format_code(dir: &Path) -> Result<()> {
    // Try to format all .rs files in the directory
    let entries = fs::read_dir(dir)
        .with_context(|| format!("Failed to read directory: {}", dir.display()))?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            format_file(&path)?;
        }
    }

    Ok(())
}

/// Format a single file with rustfmt
fn format_file(path: &Path) -> Result<()> {
    let output = Command::new("rustfmt")
        .arg(path)
        .output()
        .context("Failed to execute rustfmt - is it installed?")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("rustfmt failed: {}", stderr);
    }

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::fs;
//     use tempfile::TempDir;

//     #[test]
//     fn test_write_file() {
//         let temp_dir = TempDir::new().unwrap();
//         let file_path = temp_dir.path().join("test.rs");

//         write_file(&file_path, "fn main() {}").unwrap();

//         let content = fs::read_to_string(&file_path).unwrap();
//         assert_eq!(content, "fn main() {}");
//     }

//     #[test]
//     fn test_create_mod_file() {
//         let temp_dir = TempDir::new().unwrap();
//         let mut files = HashMap::new();
//         files.insert("patient".to_string(), String::new());
//         files.insert("observation".to_string(), String::new());

//         create_mod_file(temp_dir.path(), &files).unwrap();

//         let content = fs::read_to_string(temp_dir.path().join("mod.rs")).unwrap();
//         assert!(content.contains("pub mod types;"));
//         assert!(content.contains("pub mod observation;"));
//         assert!(content.contains("pub mod patient;"));
//     }
// }
