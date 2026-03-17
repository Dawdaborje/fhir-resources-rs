# FHIR R5 Code Generator - Architecture

## Module Structure

```
crates/codegen/
  src/
    main.rs                      # CLI entry point, version dispatcher
    common/                      # Shared utilities (reused from R4B)
      mod.rs                     # Module declarations
      codegen.rs                 # Version-agnostic code generation
      type_mapper.rs             # FHIR to Rust type mapping
      writer.rs                  # File I/O and rustfmt integration
    r5_generator/
      mod.rs                     # Public generate() function, orchestration
      parser.rs                  # Parse FHIR R5 JSON Bundle
```

## Component Responsibilities

### CLI Entry Point (`main.rs`)

**Purpose**: Parse command-line arguments and dispatch to version-specific generator

**Updated Interface**:
```rust
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        bail!(
            "Usage: {} <version>\n  Supported versions: r4, r4b, r5, r6",
            args[0]
        );
    }

    let version = &args[1];

    println!("Generating FHIR {} types...", version);

    match version.as_str() {
        "r4" => r4_generator::generate()?,
        "r4b" => r4b_generator::generate()?,
        "r5" => r5_generator::generate()?,         // NEW
        "r6" => bail!("FHIR R6 is not yet supported"),
        _ => bail!(
            "Unknown FHIR version: {}. Supported versions: r4, r4b, r5, r6",
            version
        ),
    }

    println!("✓ Code generation complete!");

    Ok(())
}
```

**Responsibilities**:
- Parse CLI arguments
- Validate version argument
- Dispatch to R5 generator when version is "r5"
- Return error codes on failure

---

### R5 Generator Module (`r5_generator/mod.rs`)

**Purpose**: Orchestrate the R5 code generation pipeline

**Interface**:
```rust
//! FHIR R5 Code Generator
//!
//! Generates Rust types from FHIR R5 schema definitions.

use anyhow::{Context, Result};
use std::path::Path;

mod parser;

/// Generate Rust types from FHIR R5 schemas
pub fn generate() -> Result<()> {
    println!("  Parsing FHIR R5 schemas...");

    // Define paths - R5 schemas are directly in schemas/r5/ (like R4)
    let schema_dir = Path::new("schemas/r5");
    let output_dir = Path::new("crates/resources/src/r5");

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

    // Generate code using common modules
    println!("  Generating Rust code...");

    let type_code = crate::common::codegen::generate_types(&types, "r5")?;
    let resource_files = crate::common::codegen::generate_resources(&resources, "r5")?;

    println!("    Generated {} resource files", resource_files.len());

    // Write files
    println!("  Writing files...");

    crate::common::writer::write_module(output_dir, type_code, resource_files, "r5")
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
```

**Key Differences from R4B**:
- Schema path is `schemas/r5` (not `schemas/r4b/definitions.json`)
- Output path is `crates/resources/src/r5`
- Version string passed to common modules is "r5"

**Responsibilities**:
- Define input/output paths specific to R5
- Coordinate parser → mapper → generator → writer pipeline
- Handle high-level errors with context
- Log progress to console

---

### Parser (`r5_generator/parser.rs`)

**Purpose**: Parse FHIR R5 JSON schemas into intermediate representation

**Interface**:
```rust
use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Bundle {
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    pub r#type: String,
    pub entry: Vec<BundleEntry>,
}

#[derive(Debug, Deserialize)]
pub struct BundleEntry {
    #[serde(rename = "fullUrl")]
    pub full_url: String,
    pub resource: StructureDefinition,
}

#[derive(Debug, Deserialize)]
pub struct StructureDefinition {
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    pub id: String,
    pub name: String,
    pub kind: String,
    pub r#type: String,
    pub snapshot: Snapshot,
}

#[derive(Debug, Deserialize)]
pub struct Snapshot {
    pub element: Vec<Element>,
}

#[derive(Debug, Deserialize)]
pub struct Element {
    pub id: String,
    pub path: String,
    pub min: Option<u32>,
    pub max: Option<String>,
    #[serde(rename = "type")]
    pub element_type: Option<Vec<ElementType>>,
    pub short: Option<String>,
    pub definition: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ElementType {
    pub code: String,
}

/// Parse a FHIR R5 Bundle file and extract StructureDefinitions
pub fn parse_bundle(path: &Path) -> Result<Vec<StructureDefinition>> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path.display()))?;

    let bundle: Bundle = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse JSON from {}", path.display()))?;

    if bundle.resource_type != "Bundle" {
        anyhow::bail!(
            "Expected resourceType 'Bundle', got '{}'",
            bundle.resource_type
        );
    }

    Ok(bundle.entry.into_iter().map(|e| e.resource).collect())
}
```

**Key Points**:
- Reuses the same Bundle/StructureDefinition structure as R4B
- R5 schemas should have same JSON structure as R4/R4B
- If R5 has new fields, they can be added to the structs with `#[serde(default)]`

**Responsibilities**:
- Read JSON files from disk
- Deserialize into typed Rust structures
- Validate bundle format
- Extract StructureDefinition entries
- Return structured data for code generation

**Potential R5-Specific Extensions**:
- If R5 introduces new fields in StructureDefinition, add them with `#[serde(default)]`
- If R5 uses different snapshot structures, adapt accordingly
- Handle R5-specific element properties

---

### Common Modules (Shared with R4 and R4B)

The following modules are shared across all FHIR versions:

#### Code Generator (`common/codegen.rs`)

**Purpose**: Generate Rust source code from StructureDefinitions

**Interface** (version-agnostic):
```rust
pub fn generate_types(
    structure_defs: &[StructureDefinition],
    version: &str,
) -> Result<String> {
    // Generate types.rs content with all complex types
}

pub fn generate_resources(
    structure_defs: &[StructureDefinition],
    version: &str,
) -> Result<HashMap<String, String>> {
    // Generate HashMap of filename → file content for all resources
}
```

**Relevant for R5**:
- Version parameter ("r5") used in documentation comments
- May need to handle R5-specific types not present in R4
- Choice type handling should work for R5 as well

---

#### Type Mapper (`common/type_mapper.rs`)

**Purpose**: Map FHIR types to Rust types

**Interface**:
```rust
pub fn map_fhir_type(fhir_type: &str) -> String {
    match fhir_type {
        "string" => "String",
        "boolean" => "bool",
        "integer" => "i32",
        "decimal" => "f64",
        // ... and so on
    }
}

pub fn apply_cardinality(
    rust_type: String,
    min: u32,
    max: &str,
) -> String {
    // Returns Option<T>, Vec<T>, Option<Vec<T>>, or T
}
```

**R5 Considerations**:
- R5 may introduce new primitive types - extend mapping if needed
- Cardinality rules should remain the same
- If R5 adds new complex types, they'll be handled as struct names

---

#### Writer (`common/writer.rs`)

**Purpose**: Write generated code to files and format with rustfmt

**Interface**:
```rust
pub fn write_module(
    output_dir: &Path,
    type_code: String,
    resource_files: HashMap<String, String>,
    version: &str,
) -> Result<()> {
    // Create output_dir if needed
    // Write types.rs
    // Write each resource file
    // Generate mod.rs with declarations
}

pub fn format_code(dir: &Path) -> Result<()> {
    // Run rustfmt on directory
}
```

**R5 Notes**:
- Output directory will be `crates/resources/src/r5`
- Same file structure as R4/R4B
- No version-specific handling needed

---

## Data Flow

```
┌─────────────────────────────────────────────────────────┐
│ 1. CLI Entry (main.rs)                                  │
│    - Parse args                                          │
│    - Dispatch to r5_generator::generate()               │
└─────────────────┬───────────────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────────────────┐
│ 2. R5 Generator (r5_generator/mod.rs)                   │
│    - Define paths (schemas/r5, crates/resources/src/r5) │
│    - Call parser                                         │
└─────────────────┬───────────────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────────────────┐
│ 3. Parser (r5_generator/parser.rs)                      │
│    - Read profiles-resources.json                        │
│    - Read profiles-types.json                            │
│    - Deserialize to StructureDefinition[]               │
└─────────────────┬───────────────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────────────────┐
│ 4. Code Generator (common/codegen.rs)                   │
│    - Iterate over StructureDefinitions                  │
│    - For each element, map types (common/type_mapper)   │
│    - Generate Rust struct/enum definitions              │
│    - Return type_code + resource_files HashMap          │
└─────────────────┬───────────────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────────────────┐
│ 5. Writer (common/writer.rs)                            │
│    - Write types.rs                                      │
│    - Write patient.rs, observation.rs, etc.             │
│    - Generate mod.rs with pub mod declarations          │
│    - Run rustfmt                                         │
└─────────────────────────────────────────────────────────┘
```

## Error Handling

Use `anyhow` for error handling throughout:

```rust
use anyhow::{Context, Result, bail};

// Add context to errors
let content = fs::read_to_string(path)
    .with_context(|| format!("Failed to read {}", path.display()))?;

// Early return with custom error
if !path.exists() {
    bail!("Schema file not found: {}", path.display());
}
```

## Logging

Use `println!` for user-facing progress messages:

```rust
println!("  Parsing FHIR R5 schemas...");
println!("    Parsed {} resources and {} types", resources.len(), types.len());
println!("  Generating Rust code...");
println!("    Generated {} resource files", resource_files.len());
println!("  Writing files...");
println!("  Formatting code with rustfmt...");
```

## Testing Strategy

### Unit Tests

Add tests in each module:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_r5_bundle() {
        let result = parse_bundle(Path::new("schemas/r5/profiles-types.json"));
        assert!(result.is_ok());
        let defs = result.unwrap();
        assert!(!defs.is_empty());
    }
}
```

### Integration Tests

Run full code generation and verify compilation:

```bash
cargo run --bin codegen -- r5
cargo check --package fhir-resources
```

### Validation Tests

Test generated code with real FHIR R5 instances:

```rust
#[test]
fn test_deserialize_r5_patient() {
    let json = r#"{ "resourceType": "Patient", "id": "example" }"#;
    let patient: r5::Patient = serde_json::from_str(json).unwrap();
    assert_eq!(patient.id, Some("example".to_string()));
}
```

## Performance Considerations

- **Large Schema Files**: R5 profiles-resources.json is ~499K lines
  - Use buffered I/O for reading
  - Consider streaming JSON parsing if memory becomes an issue
  - Current approach (read entire file) should work fine for files under 100MB

- **Code Generation Speed**: 
  - Generating hundreds of resource files may take seconds
  - Progress logging helps user understand what's happening
  - rustfmt can be slow - run only once at the end

- **Compilation Time**:
  - Generated src/r5/ will have hundreds of files
  - Incremental compilation should handle this well
  - Consider parallel rustfmt if needed
